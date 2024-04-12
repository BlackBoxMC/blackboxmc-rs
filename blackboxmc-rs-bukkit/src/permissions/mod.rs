#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct Permissible<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Permissible<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Permissible<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Permissible from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/Permissible")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Permissible object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Permissible<'mc> {
    /// Checks if this object contains an override for the specified {@link
    /// Permission}
    pub fn is_permission_set(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Gets the value of the specified permission, if set.
    ///
    /// If a permission override is not set on this object, the default value
    /// of the permission will be returned
    pub fn has_permission(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Temporarily adds a new {@link PermissionAttachment} with a single
    /// permission by name and value
    pub fn add_attachment(
        &self,
        plugin: impl Into<crate::plugin::Plugin<'mc>>,
        name: std::option::Option<impl Into<String>>,
        value: std::option::Option<bool>,
        ticks: std::option::Option<i32>,
    ) -> Result<Option<crate::permissions::PermissionAttachment<'mc>>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(plugin.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = name {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = value {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = ticks {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::permissions::PermissionAttachment::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Removes the given {@link PermissionAttachment} from this object
    pub fn remove_attachment(
        &self,
        attachment: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(attachment.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Recalculates the permissions for this object, if the attachments have
    /// changed values.
    ///
    /// This should very rarely need to be called from a plugin.
    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set containing all of the permissions currently in effect by
    /// this object
    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if this object is a server operator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the operator status of this object
    pub fn set_op(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
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
impl<'mc> Into<crate::permissions::ServerOperator<'mc>> for Permissible<'mc> {
    fn into(self) -> crate::permissions::ServerOperator<'mc> {
        crate::permissions::ServerOperator::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting Permissible into crate::permissions::ServerOperator")
    }
}
#[repr(C)]
pub struct PermissionAttachmentInfo<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionAttachmentInfo<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PermissionAttachmentInfo<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PermissionAttachmentInfo from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/permissions/PermissionAttachmentInfo")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionAttachmentInfo object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionAttachmentInfo<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        permissible: impl Into<crate::permissions::Permissible<'mc>>,
        permission: impl Into<String>,
        attachment: impl Into<crate::permissions::PermissionAttachment<'mc>>,
        value: bool,
    ) -> Result<crate::permissions::PermissionAttachmentInfo<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/Permissible;Ljava/lang/String;Lorg/bukkit/permissions/PermissionAttachment;Z)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(permissible.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(permission.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(attachment.into().jni_object().clone())
        });
        let val_4 = jni::objects::JValueGen::Bool(value.into());
        let cls = jni.find_class("org/bukkit/permissions/PermissionAttachmentInfo");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
                jni::objects::JValueGen::from(val_4),
            ],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::PermissionAttachmentInfo::from_raw(&jni, res)
    }
    /// Gets the permissible this is attached to
    pub fn permissible(
        &self,
    ) -> Result<crate::permissions::Permissible<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/Permissible;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permissible::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the permission being set
    pub fn permission(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermission", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the attachment providing this permission. This may be null for
    /// default permissions (usually parent permissions).
    pub fn attachment(
        &self,
    ) -> Result<Option<crate::permissions::PermissionAttachment<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionAttachment;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAttachment", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::permissions::PermissionAttachment::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Gets the value of this permission
    pub fn value(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getValue", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct Permission<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Permission<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Permission<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Permission from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/Permission")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Permission object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Permission<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
        description: std::option::Option<impl Into<String>>,
        default_value: std::option::Option<impl Into<crate::permissions::PermissionDefault<'mc>>>,
        children: std::option::Option<impl Into<blackboxmc_java::util::JavaMap<'mc>>>,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
        ));
        args.push(val_1);
        if let Some(a) = description {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                jni.new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = default_value {
            sig += "Lorg/bukkit/permissions/PermissionDefault;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = children {
            sig += "Ljava/util/Map;";
            let val_4 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_4);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/permissions/Permission");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::Permission::from_raw(&jni, res)
    }
    /// Returns the unique fully qualified name of this Permission
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
    /// Gets the children of this permission.
    ///
    /// If you change this map in any form, you must call {@link
    /// #recalculatePermissibles()} to recalculate all {@link Permissible}s
    pub fn children(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getChildren", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the default value of this permission.
    pub fn default(
        &self,
    ) -> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionDefault;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefault", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::PermissionDefault::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the default value of this permission.
    ///
    /// This will not be saved to disk, and is a temporary operation until the
    /// server reloads permissions. Changing this default will cause all {@link
    /// Permissible}s that contain this permission to recalculate their
    /// permissions
    pub fn set_default(
        &self,
        value: impl Into<crate::permissions::PermissionDefault<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionDefault;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(value.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefault",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a brief description of this permission, may be empty
    pub fn description(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDescription", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Sets the description of this permission.
    ///
    /// This will not be saved to disk, and is a temporary operation until the
    /// server reloads permissions.
    pub fn set_description(
        &self,
        value: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(value.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDescription",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set containing every {@link Permissible} that has this
    /// permission.
    ///
    /// This set cannot be modified.
    pub fn permissibles(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissibles", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Recalculates all {@link Permissible}s that contain this permission.
    ///
    /// This should be called after modifying the children, and is
    /// automatically called after modifying the default value
    pub fn recalculate_permissibles(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissibles",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds this permission to the specified parent permission.
    pub fn add_parent(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
        value: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Z";
        let val_2 = jni::objects::JValueGen::Bool(value.into());
        args.push(val_2);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addParent", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Loads a list of Permissions from a map of data, usually used from
    /// retrieval from a yaml file.
    ///
    /// The data may contain a list of name:data, where the data contains the
    /// following keys:
    /// <ul>
    /// <li>default: Boolean true or false. If not specified, false.
    /// <li>children: {@code Map<String, Boolean>} of child permissions. If not
    /// specified, empty list.
    /// <li>description: Short string containing a very small description of
    /// this description. If not specified, empty string.
    /// </ul>
    pub fn load_permissions(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        data: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        error: impl Into<String>,
        def: impl Into<crate::permissions::PermissionDefault<'mc>>,
    ) -> Result<Vec<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/util/Map;Ljava/lang/String;Lorg/bukkit/permissions/PermissionDefault;)Ljava/util/List;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(error.into())?,
        ));
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(def.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/permissions/Permission");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "loadPermissions",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        let res = jni.translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&jni, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::permissions::Permission::from_raw(&jni, obj)?);
        }
        Ok(new_vec)
    }
    /// Loads a Permission from a map of data, usually used from retrieval from
    /// a yaml file.
    ///
    /// The data may contain the following keys:
    /// <ul>
    /// <li>default: Boolean true or false. If not specified, false.
    /// <li>children: {@code Map<String, Boolean>} of child permissions. If not
    /// specified, empty list.
    /// <li>description: Short string containing a very small description of
    /// this description. If not specified, empty string.
    /// </ul>
    pub fn load_permission(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
        data: impl Into<blackboxmc_java::util::JavaMap<'mc>>,
        def: std::option::Option<impl Into<crate::permissions::PermissionDefault<'mc>>>,
        output: std::option::Option<Vec<jni::objects::JObject<'mc>>>,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
        ));
        args.push(val_1);
        sig += "Ljava/util/Map;";
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(data.into().jni_object().clone())
        });
        args.push(val_2);
        if let Some(a) = def {
            sig += "Lorg/bukkit/permissions/PermissionDefault;";
            let val_3 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_3);
        }
        if let Some(a) = output {
            sig += "Ljava/util/List;";
            let raw_val_4 = jni.new_object("java/util/ArrayList", "()V", vec![])?;
            for v in a {
                sig += "Ljava/lang/java/lang/Object;";
                let map_val_0 = jni::objects::JValueGen::Object(v);
                jni.call_method(
                    &raw_val_4,
                    "add",
                    "(Ljava/lang/Object;)Z",
                    vec![jni::objects::JValueGen::from(map_val_0)],
                )?;
            }
            let val_4 = jni::objects::JValueGen::Object(raw_val_4);
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/Permission;";
        let cls = jni.find_class("org/bukkit/permissions/Permission");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "loadPermission", sig.as_str(), args);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::permissions::Permission::from_raw(&jni, obj)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PermissionAttachment<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionAttachment<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PermissionAttachment<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PermissionAttachment from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/permissions/PermissionAttachment")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionAttachment object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionAttachment<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: impl Into<crate::plugin::Plugin<'mc>>,
        permissible: impl Into<crate::permissions::Permissible<'mc>>,
    ) -> Result<crate::permissions::PermissionAttachment<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/plugin/Plugin;Lorg/bukkit/permissions/Permissible;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(plugin.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(permissible.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/permissions/PermissionAttachment");
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
        crate::permissions::PermissionAttachment::from_raw(&jni, res)
    }
    /// Gets the plugin responsible for this attachment
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
    /// Sets an object to be called for when this attachment is removed from a
    /// {@link Permissible}. May be null.
    pub fn set_removal_callback(
        &self,
        ex: impl Into<crate::permissions::PermissionRemovedExecutor<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionRemovedExecutor;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(ex.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setRemovalCallback",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the class that was previously set to be called when this
    /// attachment was removed from a {@link Permissible}. May be null.
    pub fn removal_callback(
        &self,
    ) -> Result<
        Option<crate::permissions::PermissionRemovedExecutor<'mc>>,
        Box<dyn std::error::Error>,
    > {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionRemovedExecutor;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRemovalCallback",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(
            crate::permissions::PermissionRemovedExecutor::from_raw(&self.jni_ref(), unsafe {
                jni::objects::JObject::from_raw(res.l()?.clone())
            })?,
        ))
    }
    /// Gets the Permissible that this is attached to
    pub fn permissible(
        &self,
    ) -> Result<crate::permissions::Permissible<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/Permissible;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissible", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permissible::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a copy of all set permissions and values contained within this
    /// attachment.
    ///
    /// This map may be modified but will not affect the attachment, as it is a
    /// copy.
    pub fn permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Map;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getPermissions", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets a permission to the given value
    pub fn set_permission(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
        value: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += "Z";
        let val_2 = jni::objects::JValueGen::Bool(value.into());
        args.push(val_2);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "setPermission", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes the specified permission from this attachment.
    ///
    /// If the permission does not exist in this attachment, nothing will
    /// happen.
    pub fn unset_permission(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "unsetPermission", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes this attachment from its registered {@link Permissible}
    pub fn remove(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "remove", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct PermissionRemovedExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionRemovedExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PermissionRemovedExecutor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PermissionRemovedExecutor from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/permissions/PermissionRemovedExecutor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionRemovedExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionRemovedExecutor<'mc> {
    /// Called when a {@link PermissionAttachment} is removed from a {@link
    /// Permissible}
    pub fn attachment_removed(
        &self,
        attachment: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(attachment.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "attachmentRemoved",
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
#[repr(C)]
pub struct PermissibleBase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissibleBase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PermissibleBase<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PermissibleBase from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/PermissibleBase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissibleBase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissibleBase<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        opable: impl Into<crate::permissions::ServerOperator<'mc>>,
    ) -> Result<crate::permissions::PermissibleBase<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/ServerOperator;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(opable.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/permissions/PermissibleBase");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::permissions::PermissibleBase::from_raw(&jni, res)
    }

    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn set_op(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_permission_set(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "isPermissionSet", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn has_permission(
        &self,
        perm: impl Into<crate::permissions::Permission<'mc>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/permissions/Permission;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(perm.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")Z";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "hasPermission", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }

    pub fn add_attachment(
        &self,
        plugin: impl Into<crate::plugin::Plugin<'mc>>,
        name: std::option::Option<impl Into<String>>,
        value: std::option::Option<bool>,
        ticks: std::option::Option<i32>,
    ) -> Result<Option<crate::permissions::PermissionAttachment<'mc>>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/plugin/Plugin;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(plugin.into().jni_object().clone())
        });
        args.push(val_1);
        if let Some(a) = name {
            sig += "Ljava/lang/String;";
            let val_2 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_2);
        }
        if let Some(a) = value {
            sig += "Z";
            let val_3 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_3);
        }
        if let Some(a) = ticks {
            sig += "I";
            let val_4 = jni::objects::JValueGen::Int(a);
            args.push(val_4);
        }
        sig += ")Lorg/bukkit/permissions/PermissionAttachment;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "addAttachment", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::permissions::PermissionAttachment::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn remove_attachment(
        &self,
        attachment: impl Into<crate::permissions::PermissionAttachment<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/permissions/PermissionAttachment;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(attachment.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removeAttachment",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn recalculate_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn clear_permissions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("()V");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "clearPermissions",
            sig.as_str(),
            vec![],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn effective_permissions(
        &self,
    ) -> Result<blackboxmc_java::util::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/Set;");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEffectivePermissions",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::util::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::permissions::Permissible<'mc>> for PermissibleBase<'mc> {
    fn into(self) -> crate::permissions::Permissible<'mc> {
        crate::permissions::Permissible::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting PermissibleBase into crate::permissions::Permissible")
    }
}
pub enum PermissionDefault<'mc> {
    VariantTrue { inner: PermissionDefaultStruct<'mc> },
    VariantFalse { inner: PermissionDefaultStruct<'mc> },
    Op { inner: PermissionDefaultStruct<'mc> },
    NotOp { inner: PermissionDefaultStruct<'mc> },
}
impl<'mc> std::fmt::Display for PermissionDefault<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionDefault::VariantTrue { .. } => f.write_str("TRUE"),
            PermissionDefault::VariantFalse { .. } => f.write_str("FALSE"),
            PermissionDefault::Op { .. } => f.write_str("OP"),
            PermissionDefault::NotOp { .. } => f.write_str("NOT_OP"),
        }
    }
}

impl<'mc> PermissionDefault<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/permissions/PermissionDefault");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/permissions/PermissionDefault;",
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
            "TRUE" => Ok(PermissionDefault::VariantTrue {
                inner: PermissionDefaultStruct::from_raw(env, obj)?,
            }),
            "FALSE" => Ok(PermissionDefault::VariantFalse {
                inner: PermissionDefaultStruct::from_raw(env, obj)?,
            }),
            "OP" => Ok(PermissionDefault::Op {
                inner: PermissionDefaultStruct::from_raw(env, obj)?,
            }),
            "NOT_OP" => Ok(PermissionDefault::NotOp {
                inner: PermissionDefaultStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct PermissionDefaultStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for PermissionDefault<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::VariantTrue { inner } => inner.0.clone(),
            Self::VariantFalse { inner } => inner.0.clone(),
            Self::Op { inner } => inner.0.clone(),
            Self::NotOp { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::VariantTrue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VariantFalse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Op { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::NotOp { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PermissionDefault<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PermissionDefault from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/PermissionDefault")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionDefault object, got {}",
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
                "TRUE" => Ok(PermissionDefault::VariantTrue {
                    inner: PermissionDefaultStruct::from_raw(env, obj)?,
                }),
                "FALSE" => Ok(PermissionDefault::VariantFalse {
                    inner: PermissionDefaultStruct::from_raw(env, obj)?,
                }),
                "OP" => Ok(PermissionDefault::Op {
                    inner: PermissionDefaultStruct::from_raw(env, obj)?,
                }),
                "NOT_OP" => Ok(PermissionDefault::NotOp {
                    inner: PermissionDefaultStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for PermissionDefaultStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for PermissionDefaultStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PermissionDefaultStruct from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/PermissionDefault")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PermissionDefaultStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> PermissionDefaultStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/permissions/PermissionDefault;");
        let cls = jni.find_class("org/bukkit/permissions/PermissionDefault");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::permissions::PermissionDefault::from_raw(&jni, obj)
    }
    /// Calculates the value of this PermissionDefault for the given operator
    /// value
    pub fn get_value(&self, op: bool) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("(Z)Z");
        let val_1 = jni::objects::JValueGen::Bool(op.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getValue",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Looks up a PermissionDefault by name
    pub fn get_by_name(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        name: impl Into<String>,
    ) -> Result<Option<crate::permissions::PermissionDefault<'mc>>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Ljava/lang/String;)Lorg/bukkit/permissions/PermissionDefault;");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            jni.new_string(name.into())?,
        ));
        let cls = jni.find_class("org/bukkit/permissions/PermissionDefault");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(
            cls,
            "getByName",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        let obj = res.l()?;
        Ok(Some(crate::permissions::PermissionDefault::from_raw(
            &jni, obj,
        )?))
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

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct ServerOperator<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for ServerOperator<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for ServerOperator<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServerOperator from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/permissions/ServerOperator")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServerOperator object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> ServerOperator<'mc> {
    /// Checks if this object is a server operator
    pub fn is_op(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let sig = String::from("()Z");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isOp", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z()?)
    }
    /// Sets the operator status of this object
    pub fn set_op(&self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Z)V");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setOp",
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
