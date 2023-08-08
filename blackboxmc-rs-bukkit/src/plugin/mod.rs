#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
/// Each entry here represents a particular plugin's awareness. These can be checked by using <a href="PluginDescriptionFile.html#getAwareness()"><code>PluginDescriptionFile.getAwareness()</code></a>.<a class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/util/Set.html#contains-java.lang.Object-" title="class or interface in java.util"><code>contains(flag)</code></a>.
pub struct PluginAwarenessFlags<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PluginAwarenessFlags<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginAwarenessFlags<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginAwarenessFlags from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginAwarenessFlags")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginAwarenessFlags object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the enum constant of this type with the specified name. The string must match <i>exactly</i> an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)
    pub fn value_of_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        Self::from_raw(&jni, obj)
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

    pub fn equals(
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

    pub fn describe_constable(
        &mut self,
    ) -> Result<blackboxmc_java::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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
/// This type is the runtime-container for the information in the plugin.yml. All plugins must have a respective plugin.yml. For plugins written in java using the standard plugin loader, this file must be in the root of the jar file.
/// <p>When Bukkit loads a plugin, it needs to know some basic information about it. It reads this information from a YAML file, 'plugin.yml'. This file consists of a set of attributes, each defined on a new line and with no indentation.</p>
/// <p>Every (almost* every) method corresponds with a specific entry in the plugin.yml. These are the <b>required</b> entries for every plugin.yml:</p>
/// <ul>
/// <li><a href="#getName()"><code>getName()</code></a> - <code>name</code></li>
/// <li><a href="#getVersion()"><code>getVersion()</code></a> - <code>version</code></li>
/// <li><a href="#getMain()"><code>getMain()</code></a> - <code>main</code></li>
/// </ul>
/// <p>Failing to include any of these items will throw an exception and cause the server to ignore your plugin.</p>
/// <p>This is a list of the possible yaml keys, with specific details included in the respective method documentations:</p>
/// <table border="1">
/// <caption>
/// The description of the plugin.yml layout
/// </caption>
/// <tbody>
/// <tr>
/// <th>Node</th>
/// <th>Method</th>
/// <th>Summary</th>
/// </tr>
/// <tr>
/// <td><code>name</code></td>
/// <td><a href="#getName()"><code>getName()</code></a></td>
/// <td>The unique name of plugin</td>
/// </tr>
/// <tr>
/// <td><code>provides</code></td>
/// <td><a href="#getProvides()"><code>getProvides()</code></a></td>
/// <td>The plugin APIs which this plugin provides</td>
/// </tr>
/// <tr>
/// <td><code>version</code></td>
/// <td><a href="#getVersion()"><code>getVersion()</code></a></td>
/// <td>A plugin revision identifier</td>
/// </tr>
/// <tr>
/// <td><code>main</code></td>
/// <td><a href="#getMain()"><code>getMain()</code></a></td>
/// <td>The plugin's initial class file</td>
/// </tr>
/// <tr>
/// <td><code>author</code>
///
/// <code>authors</code></td>
/// <td><a href="#getAuthors()"><code>getAuthors()</code></a></td>
/// <td>The plugin authors</td>
/// </tr>
/// <tr>
/// <td><code>contributors</code></td>
/// <td><a href="#getContributors()"><code>getContributors()</code></a></td>
/// <td>The plugin contributors</td>
/// </tr>
/// <tr>
/// <td><code>description</code></td>
/// <td><a href="#getDescription()"><code>getDescription()</code></a></td>
/// <td>Human readable plugin summary</td>
/// </tr>
/// <tr>
/// <td><code>website</code></td>
/// <td><a href="#getWebsite()"><code>getWebsite()</code></a></td>
/// <td>The URL to the plugin's site</td>
/// </tr>
/// <tr>
/// <td><code>prefix</code></td>
/// <td><a href="#getPrefix()"><code>getPrefix()</code></a></td>
/// <td>The token to prefix plugin log entries</td>
/// </tr>
/// <tr>
/// <td><code>load</code></td>
/// <td><a href="#getLoad()"><code>getLoad()</code></a></td>
/// <td>The phase of server-startup this plugin will load during</td>
/// </tr>
/// <tr>
/// <td><code>depend</code></td>
/// <td><a href="#getDepend()"><code>getDepend()</code></a></td>
/// <td>Other required plugins</td>
/// </tr>
/// <tr>
/// <td><code>softdepend</code></td>
/// <td><a href="#getSoftDepend()"><code>getSoftDepend()</code></a></td>
/// <td>Other plugins that add functionality</td>
/// </tr>
/// <tr>
/// <td><code>loadbefore</code></td>
/// <td><a href="#getLoadBefore()"><code>getLoadBefore()</code></a></td>
/// <td>The inverse softdepend</td>
/// </tr>
/// <tr>
/// <td><code>commands</code></td>
/// <td><a href="#getCommands()"><code>getCommands()</code></a></td>
/// <td>The commands the plugin will register</td>
/// </tr>
/// <tr>
/// <td><code>permissions</code></td>
/// <td><a href="#getPermissions()"><code>getPermissions()</code></a></td>
/// <td>The permissions the plugin will register</td>
/// </tr>
/// <tr>
/// <td><code>default-permission</code></td>
/// <td><a href="#getPermissionDefault()"><code>getPermissionDefault()</code></a></td>
/// <td>The default <a href="../permissions/Permission.html#getDefault()"><code>default</code></a> permission state for defined <a href="#getPermissions()"><code>permissions</code></a> the plugin will register</td>
/// </tr>
/// <tr>
/// <td><code>awareness</code></td>
/// <td><a href="#getAwareness()"><code>getAwareness()</code></a></td>
/// <td>The concepts that the plugin acknowledges</td>
/// </tr>
/// <tr>
/// <td><code>api-version</code></td>
/// <td><a href="#getAPIVersion()"><code>getAPIVersion()</code></a></td>
/// <td>The API version which this plugin was programmed against</td>
/// </tr>
/// <tr>
/// <td><code>libraries</code></td>
/// <td><a href="#getLibraries()"><code>()</code></a></td>
/// <td>The libraries to be linked with this plugin</td>
/// </tr>
/// </tbody>
/// </table>
/// <p>A plugin.yml example:</p>
/// <blockquote>
/// <pre>name: Inferno
/// provides: [Hell]
/// version: 1.4.1
/// description: This plugin is so 31337. You can set yourself on fire.
/// # We could place every author in the authors list, but chose not to for illustrative purposes
/// # Also, having an author distinguishes that person as the project lead, and ensures their
/// # name is displayed first
/// author: CaptainInflamo
/// authors: [Cogito, verrier, EvilSeph]
/// contributors: [Choco, md_5]
/// website: http://www.curse.com/server-mods/minecraft/myplugin
/// main: com.captaininflamo.bukkit.inferno.Inferno
/// depend: [NewFire, FlameWire]
/// api-version: 1.13
/// libraries:
/// - com.squareup.okhttp3:okhttp:4.9.0
/// commands:
/// flagrate:
/// description: Set yourself on fire.
/// aliases: [combust_me, combustMe]
/// permission: inferno.flagrate
/// usage: Syntax error! Simply type /&lt;command&gt; to ignite yourself.
/// burningdeaths:
/// description: List how many times you have died by fire.
/// aliases: [burning_deaths, burningDeaths]
/// permission: inferno.burningdeaths
/// usage: |
/// /&lt;command&gt; [player]
/// Example: /&lt;command&gt; - see how many times you have burned to death
/// Example: /&lt;command&gt; CaptainIce - see how many times CaptainIce has burned to death
/// permissions:
/// inferno.*:
/// description: Gives access to all Inferno commands
/// children:
/// inferno.flagrate: true
/// inferno.burningdeaths: true
/// inferno.burningdeaths.others: true
/// inferno.flagrate:
/// description: Allows you to ignite yourself
/// default: true
/// inferno.burningdeaths:
/// description: Allows you to see how many times you have burned to death
/// default: true
/// inferno.burningdeaths.others:
/// description: Allows you to see how many times others have burned to death
/// default: op
/// children:
/// inferno.burningdeaths: true
/// </pre>
/// </blockquote>
pub struct PluginDescriptionFile<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PluginDescriptionFile<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginDescriptionFile<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate PluginDescriptionFile from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginDescriptionFile")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginDescriptionFile object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub unsafe fn new_with_reader(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let cls = &jni.find_class("org/bukkit/plugin/PluginDescriptionFile")?;
        let res = jni.new_object(
            cls,
            "(Ljava/io/InputStream;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        )?;
        crate::plugin::PluginDescriptionFile::from_raw(&jni, res)
    }
    pub fn new_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
        arg2: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.into()).unwrap());
        let val_3 = jni::objects::JObject::from(jni.new_string(arg2.unwrap().into()).unwrap());
        let cls = &jni.find_class("org/bukkit/plugin/PluginDescriptionFile")?;
        let res = jni.new_object(
            cls,
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
                jni::objects::JValueGen::from(&val_3),
            ],
        )?;
        crate::plugin::PluginDescriptionFile::from_raw(&jni, res)
    }
    /// Returns the name of a plugin, including the version. This method is provided for convenience; it uses the <a href="#getName()"><code>getName()</code></a> and <a href="#getVersion()"><code>getVersion()</code></a> entries.
    pub fn full_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getFullName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives the version of the plugin.
    /// <ul>
    /// <li>Version is an arbitrary string, however the most common format is MajorRelease.MinorRelease.Build (eg: 1.4.1).</li>
    /// <li>Typically you will increment this every time you release a new feature or bug fix.</li>
    /// <li>Displayed when a user types <code>/version PluginName</code></li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>version</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>version: 1.4.1</pre>
    /// </blockquote>
    pub fn version(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getVersion",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// unused
    /// </div>
    /// unused
    ///
    pub fn class_loader_of(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getClassLoaderOf",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives a human-friendly description of the functionality the plugin provides.
    /// <ul>
    /// <li>The description can have multiple lines.</li>
    /// <li>Displayed when a user types <code>/version PluginName</code></li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>description</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>description: This plugin is so 31337. You can set yourself on fire.</pre>
    /// </blockquote>
    pub fn description(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives the token to prefix plugin-specific logging messages with.
    /// <ul>
    /// <li>This includes all messages using <a href="Plugin.html#getLogger()"><code>Plugin.getLogger()</code></a>.</li>
    /// <li>If not specified, the server uses the plugin's <a href="#getName()"><code>name</code></a>.</li>
    /// <li>This should clearly indicate what plugin is being logged.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>prefix</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>prefix: ex-why-zee</pre>
    /// </blockquote>
    pub fn prefix(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPrefix",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives the list of other plugin APIs which this plugin provides. These are usable for other plugins to depend on.
    /// <ul>
    /// <li>Must consist of all alphanumeric characters, underscores, hyphon, and period (a-z,A-Z,0-9, _.-). Any other character will cause the plugin.yml to fail loading.</li>
    /// <li>A different plugin providing the same one or using it as their name will not result in the plugin to fail loading.</li>
    /// <li>Case sensitive.</li>
    /// <li>An entry of this list can be referenced in <a href="#getDepend()"><code>getDepend()</code></a>, <a href="#getSoftDepend()"><code>getSoftDepend()</code></a>, and <a href="#getLoadBefore()"><code>getLoadBefore()</code></a>.</li>
    /// <li><code>provides</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>provides</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>provides:
    /// - OtherPluginName
    /// - OldPluginName</pre>
    /// </blockquote>
    pub fn provides(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getProvides",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gives the fully qualified name of the main class for a plugin. The format should follow the <a title="class or interface in java.lang" class="external-link" href="https://docs.oracle.com/javase/8/docs/api/java/lang/ClassLoader.html#loadClass-java.lang.String-"><code>ClassLoader.loadClass(String)</code></a> syntax to successfully be resolved at runtime. For most plugins, this is the class that extends <a href="java/JavaPlugin.html" title="class in org.bukkit.plugin.java"><code>JavaPlugin</code></a>.
    /// <ul>
    /// <li>This must contain the full namespace including the class file itself.</li>
    /// <li>If your namespace is <code>org.bukkit.plugin</code>, and your class file is called <code>MyPlugin</code> then this must be <code>org.bukkit.plugin.MyPlugin</code></li>
    /// <li>No plugin can use <code>org.bukkit.</code> as a base package for <b>any class</b>, including the main class.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>main</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>main: org.bukkit.plugin.MyPlugin</pre>
    /// </blockquote>
    pub fn main(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getMain", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives the phase of server startup that the plugin should be loaded.
    /// <ul>
    /// <li>Possible values are in <a title="enum in org.bukkit.plugin" href="PluginLoadOrder.html"><code>PluginLoadOrder</code></a>.</li>
    /// <li>Defaults to <a href="PluginLoadOrder.html#POSTWORLD"><code>PluginLoadOrder.POSTWORLD</code></a>.</li>
    /// <li>Certain caveats apply to each phase.</li>
    /// <li>When different, <a href="#getDepend()"><code>getDepend()</code></a>, <a href="#getSoftDepend()"><code>getSoftDepend()</code></a>, and <a href="#getLoadBefore()"><code>getLoadBefore()</code></a> become relative in order loaded per-phase. If a plugin loads at <code>STARTUP</code>, but a dependency loads at <code>POSTWORLD</code>, the dependency will not be loaded before the plugin is loaded.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>load</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>load: STARTUP</pre>
    /// </blockquote>
    /// Gets the list of plugins that should consider this plugin a soft-dependency.
    /// <ul>
    /// <li>Use the value in the <a href="#getName()"><code>getName()</code></a> of the target plugin to specify the dependency.</li>
    /// <li>The plugin should load before any other plugins listed here.</li>
    /// <li>Specifying another plugin here is strictly equivalent to having the specified plugin's <a href="#getSoftDepend()"><code>getSoftDepend()</code></a> include <a href="#getName()"><code>this plugin</code></a>.</li>
    /// <li><code>loadbefore</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>loadbefore</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>loadbefore:
    /// - OnePlugin
    /// - AnotherPlugin</pre>
    /// </blockquote>
    pub fn load(
        &mut self,
    ) -> Result<crate::plugin::PluginLoadOrder<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLoad",
            "()Lorg/bukkit/plugin/PluginLoadOrder;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::plugin::PluginLoadOrder::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::plugin::PluginLoadOrder::from_string(variant_str).unwrap(),
        )
    }
    /// Gives the list of authors for the plugin.
    /// <ul>
    /// <li>Gives credit to the developer.</li>
    /// <li>Used in some server error messages to provide helpful feedback on who to contact when an error occurs.</li>
    /// <li>A SpigotMC forum handle or email address is recommended.</li>
    /// <li>Is displayed when a user types <code>/version PluginName</code></li>
    /// <li><code>authors</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this has two entries, <code>author</code> and <code>authors</code>.</p>
    /// <p>Single author example:</p>
    /// <blockquote>
    /// <pre>author: CaptainInflamo</pre>
    /// </blockquote> Multiple author example:
    /// <blockquote>
    /// <pre>authors: [Cogito, verrier, EvilSeph]</pre>
    /// </blockquote> When both are specified, author will be the first entry in the list, so this example:
    /// <blockquote>
    /// <pre>author: Grum
    /// authors:
    /// - feildmaster
    /// - amaranth</pre>
    /// </blockquote> Is equivilant to this example:
    /// <pre>authors: [Grum, feildmaster, aramanth]</pre>
    pub fn authors(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getAuthors", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gives the list of contributors for the plugin.
    /// <ul>
    /// <li>Gives credit to those that have contributed to the plugin, though not enough so to warrant authorship.</li>
    /// <li>Unlike <a href="#getAuthors()"><code>getAuthors()</code></a>, contributors will not be mentioned in server error messages as a means of contact.</li>
    /// <li>A SpigotMC forum handle or email address is recommended.</li>
    /// <li>Is displayed when a user types <code>/version PluginName</code></li>
    /// <li><code>contributors</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>authors: [Choco, md_5]</pre>
    /// </blockquote>
    pub fn contributors(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getContributors",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gives the plugin's or plugin's author's website.
    /// <ul>
    /// <li>A link to the Curse page that includes documentation and downloads is highly recommended.</li>
    /// <li>Displayed when a user types <code>/version PluginName</code></li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>website</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>website: http://www.curse.com/server-mods/minecraft/myplugin</pre>
    /// </blockquote>
    pub fn website(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getWebsite",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives a list of other plugins that the plugin requires.
    /// <ul>
    /// <li>Use the value in the <a href="#getName()"><code>getName()</code></a> of the target plugin to specify the dependency.</li>
    /// <li>If any plugin listed here is not found, your plugin will fail to load at startup.</li>
    /// <li>If multiple plugins list each other in <code>depend</code>, creating a network with no individual plugin does not list another plugin in the <a href="https://en.wikipedia.org/wiki/Circular_dependency">network</a>, all plugins in that network will fail.</li>
    /// <li><code>depend</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>depend</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>depend:
    /// - OnePlugin
    /// - AnotherPlugin</pre>
    /// </blockquote>
    pub fn depend(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDepend", "()Ljava/util/List;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gives a list of other plugins that the plugin requires for full functionality. The <a href="PluginManager.html" title="interface in org.bukkit.plugin"><code>PluginManager</code></a> will make best effort to treat all entries here as if they were a <a href="#getDepend()"><code>dependency</code></a>, but will never fail because of one of these entries.
    /// <ul>
    /// <li>Use the value in the <a href="#getName()"><code>getName()</code></a> of the target plugin to specify the dependency.</li>
    /// <li>When an unresolvable plugin is listed, it will be ignored and does not affect load order.</li>
    /// <li>When a circular dependency occurs (a network of plugins depending or soft-dependending each other), it will arbitrarily choose a plugin that can be resolved when ignoring soft-dependencies.</li>
    /// <li><code>softdepend</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>softdepend</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>softdepend: [OnePlugin, AnotherPlugin]</pre>
    /// </blockquote>
    pub fn soft_depend(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getSoftDepend",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gets the list of plugins that should consider this plugin a soft-dependency.
    /// <ul>
    /// <li>Use the value in the <a href="#getName()"><code>getName()</code></a> of the target plugin to specify the dependency.</li>
    /// <li>The plugin should load before any other plugins listed here.</li>
    /// <li>Specifying another plugin here is strictly equivalent to having the specified plugin's <a href="#getSoftDepend()"><code>getSoftDepend()</code></a> include <a href="#getName()"><code>this plugin</code></a>.</li>
    /// <li><code>loadbefore</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>loadbefore</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>loadbefore:
    /// - OnePlugin
    /// - AnotherPlugin</pre>
    /// </blockquote>
    pub fn load_before(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLoadBefore",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Gives the map of command-name to command-properties. Each entry in this map corresponds to a single command and the respective values are the properties of the command. Each property, <i>with the exception of aliases</i>, can be defined at runtime using methods in <a title="class in org.bukkit.command" href="../command/PluginCommand.html"><code>PluginCommand</code></a> and are defined here only as a convenience.
    /// <table border="1">
    /// <caption>
    /// The command section's description
    /// </caption>
    /// <tbody>
    /// <tr>
    /// <th>Node</th>
    /// <th>Method</th>
    /// <th>Type</th>
    /// <th>Description</th>
    /// <th>Example</th>
    /// </tr>
    /// <tr>
    /// <td><code>description</code></td>
    /// <td><a href="../command/Command.html#setDescription(java.lang.String)"><code>Command.setDescription(String)</code></a></td>
    /// <td>String</td>
    /// <td>A user-friendly description for a command. It is useful for documentation purposes as well as in-game help.</td>
    /// <td>
    /// <blockquote>
    /// <pre>description: Set yourself on fire</pre>
    /// </blockquote></td>
    /// </tr>
    /// <tr>
    /// <td><code>aliases</code></td>
    /// <td><a href="../command/Command.html#setAliases(java.util.List)"><code>Command.setAliases(List)</code></a></td>
    /// <td>String or <a href="http://en.wikipedia.org/wiki/YAML#Lists">List</a> of strings</td>
    /// <td>Alternative command names, with special usefulness for commands that are already registered. <i>Aliases are not effective when defined at runtime,</i> so the plugin description file is the only way to have them properly defined.
    /// <p>Note: Command aliases may not have a colon in them.</p></td>
    /// <td>Single alias format:
    /// <blockquote>
    /// <pre>aliases: combust_me</pre>
    /// </blockquote> or multiple alias format:
    /// <blockquote>
    /// <pre>aliases: [combust_me, combustMe]</pre>
    /// </blockquote></td>
    /// </tr>
    /// <tr>
    /// <td><code>permission</code></td>
    /// <td><a href="../command/Command.html#setPermission(java.lang.String)"><code>Command.setPermission(String)</code></a></td>
    /// <td>String</td>
    /// <td>The name of the <a href="../permissions/Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a> required to use the command. A user without the permission will receive the specified message (see <a href="../command/Command.html#setPermissionMessage(java.lang.String)">below</a>), or a standard one if no specific message is defined. Without the permission node, no <a href="../command/PluginCommand.html#setExecutor(org.bukkit.command.CommandExecutor)"><code>CommandExecutor</code></a> or <a href="../command/PluginCommand.html#setTabCompleter(org.bukkit.command.TabCompleter)"><code>PluginCommand.setTabCompleter(TabCompleter)</code></a> will be called.</td>
    /// <td>
    /// <blockquote>
    /// <pre>permission: inferno.flagrate</pre>
    /// </blockquote></td>
    /// </tr>
    /// <tr>
    /// <td><code>permission-message</code></td>
    /// <td><a href="../command/Command.html#setPermissionMessage(java.lang.String)"><code>Command.setPermissionMessage(String)</code></a></td>
    /// <td>String</td>
    /// <td>
    /// <ul>
    /// <li>Displayed to a player that attempts to use a command, but does not have the required permission. See <a href="../command/Command.html#getPermission()"><code>above</code></a>.</li>
    /// <li>&lt;permission&gt; is a macro that is replaced with the permission node required to use the command.</li>
    /// <li>Using empty quotes is a valid way to indicate nothing should be displayed to a player.</li>
    /// </ul></td>
    /// <td>
    /// <blockquote>
    /// <pre>permission-message: You do not have /&lt;permission&gt;</pre>
    /// </blockquote></td>
    /// </tr>
    /// <tr>
    /// <td><code>usage</code></td>
    /// <td><a href="../command/Command.html#setUsage(java.lang.String)"><code>Command.setUsage(String)</code></a></td>
    /// <td>String</td>
    /// <td>This message is displayed to a player when the <a href="../command/PluginCommand.html#setExecutor(org.bukkit.command.CommandExecutor)"><code>PluginCommand.setExecutor(CommandExecutor)</code></a> <a href="../command/CommandExecutor.html#onCommand(org.bukkit.command.CommandSender,org.bukkit.command.Command,java.lang.String,java.lang.String%5B%5D)">returns false</a>. &lt;command&gt; is a macro that is replaced the command issued.</td>
    /// <td>
    /// <blockquote>
    /// <pre>usage: Syntax error! Perhaps you meant /&lt;command&gt; PlayerName?</pre>
    /// </blockquote> It is worth noting that to use a colon in a yaml, like <code>`usage: Usage: /god [player]'</code>, you need to <a href="http://yaml.org/spec/current.html#id2503232">surround the message with double-quote</a>:
    /// <blockquote>
    /// <pre>usage: "Usage: /god [player]"</pre>
    /// </blockquote></td>
    /// </tr>
    /// </tbody>
    /// </table> The commands are structured as a hiearchy of <a href="http://yaml.org/spec/current.html#id2502325">nested mappings</a>. The primary (top-level, no intendentation) node is `<code>commands</code>', while each individual command name is indented, indicating it maps to some value (in our case, the properties of the table above).
    /// <p>Here is an example bringing together the piecemeal examples above, as well as few more definitions:</p>
    /// <blockquote>
    /// <pre>commands:
    /// flagrate:
    /// description: Set yourself on fire.
    /// aliases: [combust_me, combustMe]
    /// permission: inferno.flagrate
    /// permission-message: You do not have /&lt;permission&gt;
    /// usage: Syntax error! Perhaps you meant /&lt;command&gt; PlayerName?
    /// burningdeaths:
    /// description: List how many times you have died by fire.
    /// aliases:
    /// - burning_deaths
    /// - burningDeaths
    /// permission: inferno.burningdeaths
    /// usage: |
    /// /&lt;command&gt; [player]
    /// Example: /&lt;command&gt; - see how many times you have burned to death
    /// Example: /&lt;command&gt; CaptainIce - see how many times CaptainIce has burned to death
    /// # The next command has no description, aliases, etc. defined, but is still valid
    /// # Having an empty declaration is useful for defining the description, permission, and messages from a configuration dynamically
    /// apocalypse:
    /// </pre>
    /// </blockquote> Note: Command names may not have a colon in their name.
    pub fn commands(
        &mut self,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getCommands", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gives the default <a href="../permissions/Permission.html#getDefault()"><code>default</code></a> state of <a href="#getPermissions()"><code>permissions</code></a> registered for the plugin.
    /// <ul>
    /// <li>If not specified, it will be <a href="../permissions/PermissionDefault.html#OP"><code>PermissionDefault.OP</code></a>.</li>
    /// <li>It is matched using <a href="../permissions/PermissionDefault.html#getByName(java.lang.String)"><code>PermissionDefault.getByName(String)</code></a></li>
    /// <li>It only affects permissions that do not define the <code>default</code> node.</li>
    /// <li>It may be any value in <a href="../permissions/PermissionDefault.html" title="enum in org.bukkit.permissions"><code>PermissionDefault</code></a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>default-permission</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>default-permission: NOT_OP</pre>
    /// </blockquote>
    pub fn permission_default(
        &mut self,
    ) -> Result<crate::permissions::PermissionDefault<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissionDefault",
            "()Lorg/bukkit/permissions/PermissionDefault;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::permissions::PermissionDefault::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::permissions::PermissionDefault::from_string(variant_str).unwrap(),
        )
    }
    /// Gives a set of every <a href="PluginAwareness.html" title="interface in org.bukkit.plugin"><code>PluginAwareness</code></a> for a plugin. An awareness dictates something that a plugin developer acknowledges when the plugin is compiled. Some implementions may define extra awarenesses that are not included in the API. Any unrecognized awareness (one unsupported or in a future version) will cause a dummy object to be created instead of failing.
    /// <ul>
    /// <li>Currently only supports the enumerated values in <a title="enum in org.bukkit.plugin" href="PluginAwareness.Flags.html"><code>PluginAwareness.Flags</code></a>.</li>
    /// <li>Each awareness starts the identifier with bang-at (<code>!@</code>).</li>
    /// <li>Unrecognized (future / unimplemented) entries are quietly replaced by a generic object that implements PluginAwareness.</li>
    /// <li>A type of awareness must be defined by the runtime and acknowledged by the API, effectively discluding any derived type from any plugin's classpath.</li>
    /// <li><code>awareness</code> must be in <a href="http://en.wikipedia.org/wiki/YAML#Lists">YAML list format</a>.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>awareness</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>awareness:
    /// - !@UTF8</pre>
    /// </blockquote>
    /// <p><b>Note:</b> Although unknown versions of some future awareness are gracefully substituted, previous versions of Bukkit (ones prior to the first implementation of awareness) will fail to load a plugin that defines any awareness.</p>
    pub fn awareness(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAwareness",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gives the API version which this plugin is designed to support. No specific format is guaranteed.
    /// <ul>
    /// <li>Refer to release notes for supported API versions.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>api-version</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>api-version: 1.13</pre>
    /// </blockquote>
    pub fn apiversion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getAPIVersion",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets the libraries this plugin requires. This is a preview feature.
    /// <ul>
    /// <li>Libraries must be GAV specifiers and are loaded from Maven Central.</li>
    /// </ul>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>libraries:
    /// - com.squareup.okhttp3:okhttp:4.9.0</pre>
    /// </blockquote>
    pub fn libraries(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLibraries",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    #[deprecated]
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// Internal use
    /// </div>
    /// Internal use
    ///
    pub fn raw_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRawName",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives the name of the plugin. This name is a unique identifier for plugins.
    /// <ul>
    /// <li>Must consist of all alphanumeric characters, underscores, hyphon, and period (a-z,A-Z,0-9, _.-). Any other character will cause the plugin.yml to fail loading.</li>
    /// <li>Used to determine the name of the plugin's data folder. Data folders are placed in the ./plugins/ directory by default, but this behavior should not be relied on. <a href="Plugin.html#getDataFolder()"><code>Plugin.getDataFolder()</code></a> should be used to reference the data folder.</li>
    /// <li>It is good practice to name your jar the same as this, for example 'MyPlugin.jar'.</li>
    /// <li>Case sensitive.</li>
    /// <li>The is the token referenced in <a href="#getDepend()"><code>getDepend()</code></a>, <a href="#getSoftDepend()"><code>getSoftDepend()</code></a>, and <a href="#getLoadBefore()"><code>getLoadBefore()</code></a>.</li>
    /// <li>Using spaces in the plugin's name is deprecated.</li>
    /// </ul>
    /// <p>In the plugin.yml, this entry is named <code>name</code>.</p>
    /// <p>Example:</p>
    /// <blockquote>
    /// <pre>name: MyPlugin</pre>
    /// </blockquote>
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gives the list of permissions the plugin will register at runtime, immediately proceding enabling. The format for defining permissions is a map from permission name to properties. To represent a map without any specific property, empty <a href="http://yaml.org/spec/current.html#id2502702">curly-braces</a> ( <code>{}</code> ) may be used (as a null value is not accepted, unlike the <a href="#getCommands()"><code>commands</code></a> above).
    /// <p>A list of optional properties for permissions:</p>
    /// <table border="1">
    /// <caption>
    /// The permission section's description
    /// </caption>
    /// <tbody>
    /// <tr>
    /// <th>Node</th>
    /// <th>Description</th>
    /// <th>Example</th>
    /// </tr>
    /// <tr>
    /// <td><code>description</code></td>
    /// <td>Plaintext (user-friendly) description of what the permission is for.</td>
    /// <td>
    /// <blockquote>
    /// <pre>description: Allows you to set yourself on fire</pre>
    /// </blockquote></td>
    /// </tr>
    /// <tr>
    /// <td><code>default</code></td>
    /// <td>The default state for the permission, as defined by <a href="../permissions/Permission.html#getDefault()"><code>Permission.getDefault()</code></a>. If not defined, it will be set to the value of <a href="#getPermissionDefault()"><code>getPermissionDefault()</code></a>.
    /// <p>For reference:</p>
    /// <ul>
    /// <li><code>true</code> - Represents a positive assignment to <a title="interface in org.bukkit.permissions" href="../permissions/Permissible.html"><code>permissibles</code></a>.</li>
    /// <li><code>false</code> - Represents no assignment to <a title="interface in org.bukkit.permissions" href="../permissions/Permissible.html"><code>permissibles</code></a>.</li>
    /// <li><code>op</code> - Represents a positive assignment to <a href="../permissions/ServerOperator.html#isOp()"><code>operator permissibles</code></a>.</li>
    /// <li><code>notop</code> - Represents a positive assignment to <a href="../permissions/ServerOperator.html#isOp()"><code>non-operator permissibiles</code></a>.</li>
    /// </ul></td>
    /// <td>
    /// <blockquote>
    /// <pre>default: true</pre>
    /// </blockquote></td>
    /// </tr>
    /// <tr>
    /// <td><code>children</code></td>
    /// <td>Allows other permissions to be set as a <a href="../permissions/Permission.html#getChildren()">relation</a> to the parent permission. When a parent permissions is assigned, child permissions are respectively assigned as well.
    /// <ul>
    /// <li>When a parent permission is assigned negatively, child permissions are assigned based on an inversion of their association.</li>
    /// <li>When a parent permission is assigned positively, child permissions are assigned based on their association.</li>
    /// </ul>
    /// <p>Child permissions may be defined in a number of ways:</p>
    /// <ul>
    /// <li>Children may be defined as a <a href="http://en.wikipedia.org/wiki/YAML#Lists">list</a> of names. Using a list will treat all children associated positively to their parent.</li>
    /// <li>Children may be defined as a map. Each permission name maps to either a boolean (representing the association), or a nested permission definition (just as another permission). Using a nested definition treats the child as a positive association.</li>
    /// <li>A nested permission definition must be a map of these same properties. To define a valid nested permission without defining any specific property, empty curly-braces ( <code>{}</code> ) must be used.</li>
    /// <li>A nested permission may carry it's own nested permissions as children, as they may also have nested permissions, and so forth. There is no direct limit to how deep the permission tree is defined.</li>
    /// </ul></td>
    /// <td>As a list:
    /// <blockquote>
    /// <pre>children: [inferno.flagrate, inferno.burningdeaths]</pre>
    /// </blockquote> Or as a mapping:
    /// <blockquote>
    /// <pre>children:
    /// inferno.flagrate: true
    /// inferno.burningdeaths: true</pre>
    /// </blockquote> An additional example showing basic nested values can be seen <a href="doc-files/permissions-example_plugin.yml">here</a>.</td>
    /// </tr>
    /// </tbody>
    /// </table> The permissions are structured as a hiearchy of <a href="http://yaml.org/spec/current.html#id2502325">nested mappings</a>. The primary (top-level, no intendentation) node is `<code>permissions</code>', while each individual permission name is indented, indicating it maps to some value (in our case, the properties of the table above).
    /// <p>Here is an example using some of the properties:</p>
    /// <blockquote>
    /// <pre>permissions:
    /// inferno.*:
    /// description: Gives access to all Inferno commands
    /// children:
    /// inferno.flagrate: true
    /// inferno.burningdeaths: true
    /// inferno.flagate:
    /// description: Allows you to ignite yourself
    /// default: true
    /// inferno.burningdeaths:
    /// description: Allows you to see how many times you have burned to death
    /// default: true
    /// </pre>
    /// </blockquote> Another example, with nested definitions, can be found <a href="doc-files/permissions-example_plugin.yml">here</a>.
    pub fn permissions(
        &mut self,
    ) -> Result<Vec<crate::permissions::Permission<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissions",
            "()Ljava/util/List;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::permissions::Permission::from_raw(&self.0, obj)?);
        }
        Ok(new_vec)
    }
    /// Saves this PluginDescriptionFile to the given writer
    pub unsafe fn save(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "save",
            "(Ljava/io/Writer;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
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

    pub fn equals(
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
    /// <span class="deprecated-label">Deprecated.</span>
    /// <div class="deprecation-comment">
    /// unused
    /// </div>
    /// unused
    ///
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
pub enum PluginLoadOrderEnum {
    Startup,
    Postworld,
}
impl std::fmt::Display for PluginLoadOrderEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PluginLoadOrderEnum::Startup => f.write_str("STARTUP"),
            PluginLoadOrderEnum::Postworld => f.write_str("POSTWORLD"),
        }
    }
}
pub struct PluginLoadOrder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PluginLoadOrderEnum,
);
impl<'mc> std::ops::Deref for PluginLoadOrder<'mc> {
    type Target = PluginLoadOrderEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PluginLoadOrder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginLoadOrder<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PluginLoadOrderEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginLoadOrder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginLoadOrder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginLoadOrder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const STARTUP: PluginLoadOrderEnum = PluginLoadOrderEnum::Startup;
    pub const POSTWORLD: PluginLoadOrderEnum = PluginLoadOrderEnum::Postworld;
    pub fn from_string(str: String) -> std::option::Option<PluginLoadOrderEnum> {
        match str.as_str() {
            "STARTUP" => Some(PluginLoadOrderEnum::Startup),
            "POSTWORLD" => Some(PluginLoadOrderEnum::Postworld),
            _ => None,
        }
    }
}
/// A registered service provider.
pub struct RegisteredServiceProvider<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RegisteredServiceProvider<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RegisteredServiceProvider<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate RegisteredServiceProvider from null object."
            )
            .into());
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/plugin/RegisteredServiceProvider")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegisteredServiceProvider object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: jni::objects::JClass<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<&'mc crate::plugin::ServicePriority<'mc>>,
        arg3: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<crate::plugin::RegisteredServiceProvider<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/plugin/RegisteredServiceProvider")?;
        let res = jni.new_object(cls,
"(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/ServicePriority;Lorg/bukkit/plugin/Plugin;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)])?;
        crate::plugin::RegisteredServiceProvider::from_raw(&jni, res)
    }

    pub fn provider(&mut self) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getProvider",
            "()Ljava/lang/Object;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn plugin(&mut self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn service(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getService",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn compare_to_with_registered_service_provider(
        &mut self,
        arg0: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "compareTo",
            "(Ljava/lang/Object;)I",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }

    pub fn priority(
        &mut self,
    ) -> Result<crate::plugin::ServicePriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriority",
            "()Lorg/bukkit/plugin/ServicePriority;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::ServicePriority::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
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

    pub fn equals(
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
/// Represents various priorities of a provider.
pub struct ServicePriority<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for ServicePriority<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> ServicePriority<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServicePriority from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/ServicePriority")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServicePriority object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the enum constant of this type with the specified name. The string must match <i>exactly</i> an identifier used to declare an enum constant in this type. (Extraneous whitespace characters are not permitted.)
    pub fn value_of_with_string(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = jni::objects::JObject::from(jni.new_string(arg1.unwrap().into()).unwrap());
        let cls = &jni.find_class("java/lang/Enum")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        )?;
        let obj = res.l()?;
        Self::from_raw(&jni, obj)
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

    pub fn equals(
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

    pub fn describe_constable(
        &mut self,
    ) -> Result<blackboxmc_java::JavaOptional<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "describeConstable",
            "()Ljava/util/Optional;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaOptional::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn declaring_class(
        &mut self,
    ) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDeclaringClass",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }

    pub fn ordinal(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "ordinal", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
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
/// Represents a concept that a plugin is aware of.
/// <p>The internal representation may be singleton, or be a parameterized instance, but must be immutable.</p>
///
/// This is a representation of an abstract class.
pub struct PluginAwareness<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginAwareness<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate PluginAwareness from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginAwareness")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginAwareness object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}
impl<'mc> JNIRaw<'mc> for PluginAwareness<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Handles all plugin management from the Server
///
/// This is a representation of an abstract class.
pub struct PluginManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginManager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginManager from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Gets a <a href="../permissions/Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a> from its fully qualified name
    /// Gets a set containing all subscribed <a href="../permissions/Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>s to the given permission, by name
    /// Gets a set of all registered permissions.
    /// <p>This set is a copy and will not be modified live.</p>
    pub fn get_permission(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::permissions::Permission<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermission",
            "(Ljava/lang/String;)Lorg/bukkit/permissions/Permission;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::permissions::Permission::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Checks if the given plugin is loaded and returns it when applicable
    /// <p>Please note that the name of the plugin is case-sensitive</p>
    /// Gets a list of all currently loaded plugins
    pub fn get_plugin(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "(Ljava/lang/String;)Lorg/bukkit/plugin/Plugin;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Calls an event with the given details
    pub fn call_event(
        &mut self,
        arg0: impl Into<&'mc crate::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "callEvent",
            "(Lorg/bukkit/event/Event;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set containing all subscribed <a href="../permissions/Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>s to the given permission, by name
    pub fn get_permission_subscriptions(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissionSubscriptions",
            "(Ljava/lang/String;)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Recalculates the defaults for the given <a href="../permissions/Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a>.
    /// <p>This will have no effect if the specified permission is not registered here.</p>
    pub fn recalculate_permission_defaults(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::Permission<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "recalculatePermissionDefaults",
            "(Lorg/bukkit/permissions/Permission;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Adds a <a href="../permissions/Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a> to this plugin manager.
    /// <p>If a permission is already defined with the given name of the new permission, an exception will be thrown.</p>
    pub fn add_permission(
        &mut self,
        arg0: impl Into<&'mc crate::permissions::Permission<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "addPermission",
            "(Lorg/bukkit/permissions/Permission;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the default permissions for the given op status
    pub fn get_default_permissions(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultPermissions",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Subscribes to the given Default permissions by operator status
    /// <p>If the specified defaults change in any form, the Permissible will be asked to recalculate.</p>
    pub fn subscribe_to_default_perms(
        &mut self,
        arg0: bool,
        arg1: impl Into<&'mc crate::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subscribeToDefaultPerms",
            "(ZLorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Subscribes the given Permissible for information about the requested Permission, by name.
    /// <p>If the specified Permission changes in any form, the Permissible will be asked to recalculate.</p>
    pub fn subscribe_to_permission(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "subscribeToPermission",
            "(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Unsubscribes the given Permissible for information about the requested Permission, by name.
    pub fn unsubscribe_from_permission(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc crate::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unsubscribeFromPermission",
            "(Ljava/lang/String;Lorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Unsubscribes from the given Default permissions by operator status
    pub fn unsubscribe_from_default_perms(
        &mut self,
        arg0: bool,
        arg1: impl Into<&'mc crate::permissions::Permissible<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unsubscribeFromDefaultPerms",
            "(ZLorg/bukkit/permissions/Permissible;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Loads the plugin in the specified file
    /// <p>File must be valid according to the current enabled Plugin interfaces</p>
    /// Loads the plugins contained within the specified directory
    pub unsafe fn load_plugin(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadPlugin",
            "(Ljava/io/File;)Lorg/bukkit/plugin/Plugin;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns whether or not timing code should be used for event calls
    pub fn use_timings(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "useTimings", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Enables the specified plugin
    /// <p>Attempting to enable a plugin that is already enabled will have no effect</p>
    pub fn enable_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "enablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Disables all the loaded plugins
    /// Disables the specified plugin
    /// <p>Attempting to disable a plugin that is not enabled will have no effect</p>
    pub fn disable_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "disablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Registers the specified plugin loader
    pub fn register_interface(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerInterface",
            "(Ljava/lang/Class;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Checks if the given plugin is enabled or not
    /// <p>Please note that the name of the plugin is case-sensitive.</p>
    /// Checks if the given plugin is enabled or not
    pub fn is_plugin_enabled_with_plugin(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isPluginEnabled",
            "(Ljava/lang/String;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Disables all the loaded plugins
    pub fn disable_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "disablePlugins", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Disables and removes all plugins
    pub fn clear_plugins(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "clearPlugins", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Registers all the events in the given listener class
    pub fn register_events(
        &mut self,
        arg0: impl Into<&'mc crate::event::Listener<'mc>>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "registerEvents",
            "(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Registers all the events in the given listener class
    /// Registers the specified executor to the given event class
    /// Registers the specified executor to the given event class
    pub fn register_event_with_class(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
        arg1: impl Into<&'mc crate::event::Listener<'mc>>,
        arg2: impl Into<&'mc crate::event::EventPriority<'mc>>,
        arg3: impl Into<&'mc crate::plugin::EventExecutor<'mc>>,
        arg4: std::option::Option<impl Into<&'mc crate::plugin::Plugin<'mc>>>,
        arg5: std::option::Option<bool>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let val_5 =
            unsafe { jni::objects::JObject::from_raw(arg4.unwrap().into().jni_object().clone()) };
        // 4
        let val_6 = jni::objects::JValueGen::Bool(arg5.unwrap().into());
        let res = self.jni_ref().call_method(&self.jni_object(),"registerEvent","(Ljava/lang/Class;Lorg/bukkit/event/Listener;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/plugin/Plugin;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5),jni::objects::JValueGen::from(&val_6)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Removes a <a href="../permissions/Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a> registration from this plugin manager.
    /// <p>If the specified permission does not exist in this plugin manager, nothing will happen.</p>
    /// <p>Removing a permission registration will <b>not</b> remove the permission from any <a title="interface in org.bukkit.permissions" href="../permissions/Permissible.html"><code>Permissible</code></a>s that have it.</p>
    /// Removes a <a href="../permissions/Permission.html" title="class in org.bukkit.permissions"><code>Permission</code></a> registration from this plugin manager.
    /// <p>If the specified permission does not exist in this plugin manager, nothing will happen.</p>
    /// <p>Removing a permission registration will <b>not</b> remove the permission from any <a href="../permissions/Permissible.html" title="interface in org.bukkit.permissions"><code>Permissible</code></a>s that have it.</p>
    pub fn remove_permission_with_permission(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc String>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 =
            jni::objects::JObject::from(self.jni_ref().new_string(arg0.unwrap().into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "removePermission",
            "(Ljava/lang/String;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a set containing all subscribed <a title="interface in org.bukkit.permissions" href="../permissions/Permissible.html"><code>Permissible</code></a>s to the given default list, by op status
    pub fn get_default_perm_subscriptions(
        &mut self,
        arg0: bool,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultPermSubscriptions",
            "(Z)Ljava/util/Set;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a set of all registered permissions.
    /// <p>This set is a copy and will not be modified live.</p>
    pub fn permissions(
        &mut self,
    ) -> Result<blackboxmc_java::JavaSet<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPermissions",
            "()Ljava/util/Set;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaSet::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
}
impl<'mc> JNIRaw<'mc> for PluginManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Represents a base <a title="interface in org.bukkit.plugin" href="Plugin.html"><code>Plugin</code></a>
/// <p>Extend this class if your plugin is not a <a href="java/JavaPlugin.html" title="class in org.bukkit.plugin.java"><code>JavaPlugin</code></a></p>
pub struct PluginBase<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for PluginBase<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PluginBase<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "PluginBase", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginBase from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginBase")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginBase object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::plugin::PluginBase<'mc>, Box<dyn std::error::Error>> {
        let cls = &jni.find_class("org/bukkit/plugin/PluginBase")?;
        let res = jni.new_object(cls, "()V", &[])?;
        crate::plugin::PluginBase::from_raw(&jni, res)
    }
    /// <span class="descfrm-type-label">Description copied from interface:&nbsp;<code><a href="Plugin.html#getName()">Plugin</a></code></span>
    /// Returns the name of the plugin.
    /// <p>This should return the bare name of the plugin and should be used for comparison.</p>
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn equals(
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

    pub fn description(
        &mut self,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Lorg/bukkit/plugin/PluginDescriptionFile;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_enabled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn get_default_biome_provider(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/BiomeProvider;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::BiomeProvider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn config(
        &mut self,
    ) -> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfig",
            "()Lorg/bukkit/configuration/file/FileConfiguration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn data_folder(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDataFolder",
            "()Ljava/io/File;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn save_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "saveConfig", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn save_default_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "saveDefaultConfig", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn save_resource(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveResource",
            "(Ljava/lang/String;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn reload_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "reloadConfig", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn plugin_loader(
        &mut self,
    ) -> Result<crate::plugin::PluginLoader<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPluginLoader",
            "()Lorg/bukkit/plugin/PluginLoader;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginLoader::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onDisable", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn on_load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onLoad", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onEnable", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn is_naggable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNaggable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn set_naggable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNaggable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn get_default_world_generator(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
    ) -> Result<crate::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultWorldGenerator",
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/ChunkGenerator;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::ChunkGenerator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn get_resource(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResource",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }

    pub fn logger(
        &mut self,
    ) -> Result<blackboxmc_java::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLogger",
            "()Ljava/util/logging/Logger;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::logging::JavaLogger::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn on_tab_complete(
        &mut self,
        arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,
        arg1: impl Into<&'mc crate::command::Command<'mc>>,
        arg2: impl Into<&'mc String>,
        arg3: Vec<impl Into<String>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let res = self.jni_ref().call_method(&self.jni_object(),"onTabComplete","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.0, res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn on_command(
        &mut self,
        arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,
        arg1: impl Into<&'mc crate::command::Command<'mc>>,
        arg2: impl Into<&'mc String>,
        arg3: Vec<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let res = self.jni_ref().call_method(&self.jni_object(),"onCommand","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> Into<crate::plugin::Plugin<'mc>> for PluginBase<'mc> {
    fn into(self) -> crate::plugin::Plugin<'mc> {
        crate::plugin::Plugin::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Interface which defines the class for event call backs to plugins
///
/// This is a representation of an abstract class.
pub struct EventExecutor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> EventExecutor<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate EventExecutor from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/EventExecutor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a EventExecutor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }

    pub fn execute(
        &mut self,
        arg0: impl Into<&'mc crate::event::Listener<'mc>>,
        arg1: impl Into<&'mc crate::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "execute",
            "(Lorg/bukkit/event/Listener;Lorg/bukkit/event/Event;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for EventExecutor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Manages services and service providers. Services are an interface specifying a list of methods that a provider must implement. Providers are implementations of these services. A provider can be queried from the services manager in order to use a service (if one is available). If multiple plugins register a service, then the service with the highest priority takes precedence.
///
/// This is a representation of an abstract class.
pub struct ServicesManager<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> ServicesManager<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate ServicesManager from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/ServicesManager")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a ServicesManager object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Unregister all the providers registered by a particular plugin.
    /// Unregister a particular provider for a particular service.
    /// Unregister a particular provider.
    pub fn unregister_with_object(
        &mut self,
        arg0: std::option::Option<jni::objects::JClass<'mc>>,
        arg1: std::option::Option<jni::objects::JObject<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0.unwrap();
        let val_2 = arg1.unwrap();
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregister",
            "(Ljava/lang/Class;Ljava/lang/Object;)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Unregister all the providers registered by a particular plugin.
    pub fn unregister_all(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "unregisterAll",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Queries for a provider registration. This may return null if no provider has been registered for a service.
    /// Get registrations of providers for a plugin.
    /// Get registrations of providers for a service. The returned list is unmodifiable.
    pub fn get_registration(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<crate::plugin::RegisteredServiceProvider<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRegistration",
            "(Ljava/lang/Class;)Lorg/bukkit/plugin/RegisteredServiceProvider;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::RegisteredServiceProvider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get registrations of providers for a plugin.
    /// Get registrations of providers for a service. The returned list is unmodifiable.
    pub fn get_registrations_with_class(
        &mut self,
        arg0: std::option::Option<impl Into<&'mc crate::plugin::Plugin<'mc>>>,
    ) -> Result<Vec<crate::plugin::RegisteredServiceProvider<'mc>>, Box<dyn std::error::Error>>
    {
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getRegistrations",
            "(Lorg/bukkit/plugin/Plugin;)Ljava/util/List;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(crate::plugin::RegisteredServiceProvider::from_raw(
                &self.jni_ref(),
                obj,
            )?);
        }
        Ok(new_vec)
    }
    /// Get a list of known services. A service is known if it has registered providers for it.
    pub fn known_services(
        &mut self,
    ) -> Result<Vec<jni::objects::JClass<'mc>>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getKnownServices",
            "()Ljava/util/Collection;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut col = blackboxmc_java::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let mut iter = blackboxmc_java::JavaIterator::from_raw(&self.jni_ref(), col.iterator()?)?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(unsafe { jni::objects::JClass::from_raw(*obj) })
        }
        Ok(new_vec)
    }
    /// Returns whether a provider has been registered for a service. Do not check this first only to call <code>load(service)</code> later, as that would be a non-thread safe situation.
    pub fn is_provided_for(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "isProvidedFor",
            "(Ljava/lang/Class;)Z",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Queries for a provider. This may return null if no provider has been registered for a service. The highest priority provider is returned.
    pub fn load(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "load",
            "(Ljava/lang/Class;)Ljava/lang/Object;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Register a provider of a service.
    pub fn register(
        &mut self,
        arg0: jni::objects::JClass<'mc>,
        arg1: jni::objects::JObject<'mc>,
        arg2: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg3: impl Into<&'mc crate::plugin::ServicePriority<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let val_2 = arg1;
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(&self.jni_object(),"register","(Ljava/lang/Class;Ljava/lang/Object;Lorg/bukkit/plugin/Plugin;Lorg/bukkit/plugin/ServicePriority;)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4)]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for ServicesManager<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Extends RegisteredListener to include timing information
pub struct TimedRegisteredListener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for TimedRegisteredListener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> TimedRegisteredListener<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate TimedRegisteredListener from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/TimedRegisteredListener")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a TimedRegisteredListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::event::Listener<'mc>>,
        arg1: impl Into<&'mc crate::plugin::EventExecutor<'mc>>,
        arg2: impl Into<&'mc crate::event::EventPriority<'mc>>,
        arg3: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg4: bool,
    ) -> Result<crate::plugin::TimedRegisteredListener<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        // -2
        let val_5 = jni::objects::JValueGen::Bool(arg4.into());
        let cls = &jni.find_class("org/bukkit/plugin/TimedRegisteredListener")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/Plugin;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::plugin::TimedRegisteredListener::from_raw(&jni, res)
    }
    /// Gets the total times this listener has been called
    pub fn count(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getCount", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    /// <span class="descfrm-type-label">Description copied from class:&nbsp;<code><a href="RegisteredListener.html#callEvent(org.bukkit.event.Event)">RegisteredListener</a></code></span>
    /// Calls the event executor
    pub fn call_event(
        &mut self,
        arg0: impl Into<&'mc crate::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "callEvent",
            "(Lorg/bukkit/event/Event;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the total time calls to this listener have taken
    pub fn total_time(&mut self) -> Result<i64, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getTotalTime", "()J", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j().unwrap())
    }
    /// Gets the class of the events this listener handled. If it handled multiple classes of event, the closest shared superclass will be returned, such that for any event this listener has handled, <code>this.getEventClass().isAssignableFrom(event.getClass())</code> and no class <code>this.getEventClass().isAssignableFrom(clazz) &amp;&amp; this.getEventClass() != clazz &amp;&amp; event.getClass().isAssignableFrom(clazz)</code> for all handled events.
    pub fn event_class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getEventClass",
            "()Ljava/lang/Class;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    /// Gets whether this listener has handled multiple events, such that for some two events, <code>eventA.getClass() != eventB.getClass()</code>.
    pub fn has_multiple(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hasMultiple", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Resets the call count and total time for this listener
    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "reset", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn listener(&mut self) -> Result<crate::event::Listener<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getListener",
            "()Lorg/bukkit/event/Listener;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::Listener::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn plugin(&mut self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn is_ignoring_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnoringCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }

    pub fn priority(
        &mut self,
    ) -> Result<crate::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriority",
            "()Lorg/bukkit/event/EventPriority;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::EventPriority::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::EventPriority::from_string(variant_str).unwrap(),
        )
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

    pub fn equals(
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
impl<'mc> Into<crate::plugin::RegisteredListener<'mc>> for TimedRegisteredListener<'mc> {
    fn into(self) -> crate::plugin::RegisteredListener<'mc> {
        crate::plugin::RegisteredListener::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
/// Represents a plugin loader, which handles direct access to specific types of plugins
///
/// This is a representation of an abstract class.
pub struct PluginLoader<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> PluginLoader<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "PluginLoader", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PluginLoader from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/PluginLoader")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PluginLoader object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Loads the plugin contained in the specified file
    pub unsafe fn load_plugin(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadPlugin",
            "(Ljava/io/File;)Lorg/bukkit/plugin/Plugin;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Loads a PluginDescriptionFile from the specified file
    pub unsafe fn get_plugin_description(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let val_1 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPluginDescription",
            "(Ljava/io/File;)Lorg/bukkit/plugin/PluginDescriptionFile;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Creates and returns registered listeners for the event classes used in this listener
    pub fn create_registered_listeners(
        &mut self,
        arg0: impl Into<&'mc crate::event::Listener<'mc>>,
        arg1: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<blackboxmc_java::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "createRegisteredListeners",
            "(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/Plugin;)Ljava/util/Map;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Enables the specified plugin
    /// <p>Attempting to enable a plugin that is already enabled will have no effect</p>
    pub fn enable_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "enablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Disables the specified plugin
    /// <p>Attempting to disable a plugin that is not enabled will have no effect</p>
    pub fn disable_plugin(
        &mut self,
        arg0: impl Into<&'mc crate::plugin::Plugin<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "disablePlugin",
            "(Lorg/bukkit/plugin/Plugin;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> JNIRaw<'mc> for PluginLoader<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
/// Stores relevant information for plugin listeners
pub struct RegisteredListener<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for RegisteredListener<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> RegisteredListener<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate RegisteredListener from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/RegisteredListener")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a RegisteredListener object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc crate::event::Listener<'mc>>,
        arg1: impl Into<&'mc crate::plugin::EventExecutor<'mc>>,
        arg2: impl Into<&'mc crate::event::EventPriority<'mc>>,
        arg3: impl Into<&'mc crate::plugin::Plugin<'mc>>,
        arg4: bool,
    ) -> Result<crate::plugin::RegisteredListener<'mc>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = unsafe { jni::objects::JObject::from_raw(arg2.into().jni_object().clone()) };
        let val_4 = unsafe { jni::objects::JObject::from_raw(arg3.into().jni_object().clone()) };
        // -2
        let val_5 = jni::objects::JValueGen::Bool(arg4.into());
        let cls = &jni.find_class("org/bukkit/plugin/RegisteredListener")?;
        let res = jni.new_object(cls,
"(Lorg/bukkit/event/Listener;Lorg/bukkit/plugin/EventExecutor;Lorg/bukkit/event/EventPriority;Lorg/bukkit/plugin/Plugin;Z)V",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3),jni::objects::JValueGen::from(&val_4),jni::objects::JValueGen::from(&val_5)])?;
        crate::plugin::RegisteredListener::from_raw(&jni, res)
    }
    /// Gets the listener for this registration
    pub fn listener(&mut self) -> Result<crate::event::Listener<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getListener",
            "()Lorg/bukkit/event/Listener;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::event::Listener::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the plugin for this registration
    pub fn plugin(&mut self) -> Result<crate::plugin::Plugin<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPlugin",
            "()Lorg/bukkit/plugin/Plugin;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::Plugin::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Calls the event executor
    pub fn call_event(
        &mut self,
        arg0: impl Into<&'mc crate::event::Event<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "callEvent",
            "(Lorg/bukkit/event/Event;)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Whether this listener accepts cancelled events
    pub fn is_ignoring_cancelled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isIgnoringCancelled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets the priority for this registration
    pub fn priority(
        &mut self,
    ) -> Result<crate::event::EventPriority<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPriority",
            "()Lorg/bukkit/event/EventPriority;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::event::EventPriority::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::event::EventPriority::from_string(variant_str).unwrap(),
        )
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

    pub fn equals(
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
/// Represents a Plugin
/// <p>The use of <a href="PluginBase.html" title="class in org.bukkit.plugin"><code>PluginBase</code></a> is recommended for actual Implementation</p>
///
/// This is a representation of an abstract class.
pub struct Plugin<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> Plugin<'mc> {
    pub fn from_extendable(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        plugin: &'mc crate::plugin::Plugin,
        address: i32,
        lib_name: String,
        name: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let obj = unsafe { plugin.new_extendable(address, "Plugin", name, lib_name) }?;
        Self::from_raw(env, obj)
    }
    /// Return one of the extendable classes that BlackBox supports, based on the value given.
    ///
    /// ## Safety
    /// - It returns a Java Object that you must then cast into the proper object via JNI. You are responsible for the checks yourself.
    /// - This function is specific to the BlackboxPlugin class supplied within the plugin, and will error out if you pass a regular JavaPlugin.
    pub unsafe fn new_extendable(
        &self,
        address: i32,
        class_name: impl Into<String>
            + std::convert::AsRef<str>
            + std::convert::AsRef<str>
            + std::convert::AsRef<str>,
        name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
        lib_name: impl Into<String> + std::convert::AsRef<str> + std::convert::AsRef<str>,
    ) -> Result<jni::objects::JObject, Box<dyn std::error::Error>> {
        let obj = self.jni_ref().call_method(
            &self.1,
            "newExtendable",
            "(ILjava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            &[
                jni::objects::JValueGen::Int(address),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    self.jni_ref().new_string(class_name).unwrap(),
                )),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    self.jni_ref().new_string(name).unwrap(),
                )),
                jni::objects::JValueGen::from(&jni::objects::JObject::from(
                    self.jni_ref().new_string(lib_name).unwrap(),
                )),
            ],
        );
        let obj = self.jni_ref().translate_error(obj)?;
        Ok(jni::objects::JObject::from_raw(*obj.l()?))
    }
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Plugin from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/plugin/Plugin")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Plugin object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    /// Returns the plugin.yaml file containing the details for this plugin
    pub fn description(
        &mut self,
    ) -> Result<crate::plugin::PluginDescriptionFile<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDescription",
            "()Lorg/bukkit/plugin/PluginDescriptionFile;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginDescriptionFile::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the Server instance currently running this plugin
    pub fn server(&mut self) -> Result<crate::Server<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getServer",
            "()Lorg/bukkit/Server;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::Server::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns a value indicating whether or not this plugin is currently enabled
    pub fn is_enabled(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isEnabled", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Gets a <a title="class in org.bukkit.generator" href="../generator/BiomeProvider.html"><code>BiomeProvider</code></a> for use in a default world, as specified in the server configuration
    pub fn get_default_biome_provider(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
    ) -> Result<crate::generator::BiomeProvider<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultBiomeProvider",
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/BiomeProvider;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::BiomeProvider::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets a <a href="../configuration/file/FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> for this plugin, read through "config.yml"
    /// <p>If there is a default config.yml embedded in this plugin, it will be provided as a default for this Configuration.</p>
    pub fn config(
        &mut self,
    ) -> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getConfig",
            "()Lorg/bukkit/configuration/file/FileConfiguration;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the folder that the plugin data's files are located in. The folder may not yet exist.
    pub fn data_folder(
        &mut self,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDataFolder",
            "()Ljava/io/File;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Saves the <a href="../configuration/file/FileConfiguration.html" title="class in org.bukkit.configuration.file"><code>FileConfiguration</code></a> retrievable by <a href="#getConfig()"><code>getConfig()</code></a>.
    pub fn save_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "saveConfig", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Saves the raw contents of the default config.yml file to the location retrievable by <a href="#getConfig()"><code>getConfig()</code></a>.
    /// <p>This should fail silently if the config.yml already exists.</p>
    pub fn save_default_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "saveDefaultConfig", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Saves the raw contents of any resource embedded with a plugin's .jar file assuming it can be found using <a href="#getResource(java.lang.String)"><code>getResource(String)</code></a>.
    /// <p>The resource is saved into the plugin's data folder using the same hierarchy as the .jar file (subdirectories are preserved).</p>
    pub fn save_resource(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        // -2
        let val_2 = jni::objects::JValueGen::Bool(arg1.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "saveResource",
            "(Ljava/lang/String;Z)V",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Discards any data in <a href="#getConfig()"><code>getConfig()</code></a> and reloads from disk.
    pub fn reload_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "reloadConfig", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the associated PluginLoader responsible for this plugin
    pub fn plugin_loader(
        &mut self,
    ) -> Result<crate::plugin::PluginLoader<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPluginLoader",
            "()Lorg/bukkit/plugin/PluginLoader;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::plugin::PluginLoader::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Called when this plugin is disabled
    pub fn on_disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onDisable", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Called after a plugin is loaded but before it has been enabled.
    /// <p>When multiple plugins are loaded, the onLoad() for all plugins is called before any onEnable() is called.</p>
    pub fn on_load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onLoad", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Called when this plugin is enabled
    pub fn on_enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "onEnable", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Simple boolean if we can still nag to the logs about things
    pub fn is_naggable(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "isNaggable", "()Z", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    /// Set naggable state
    pub fn set_naggable(&mut self, arg0: bool) -> Result<(), Box<dyn std::error::Error>> {
        // -2
        let val_1 = jni::objects::JValueGen::Bool(arg0.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setNaggable",
            "(Z)V",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets a <a title="class in org.bukkit.generator" href="../generator/ChunkGenerator.html"><code>ChunkGenerator</code></a> for use in a default world, as specified in the server configuration
    pub fn get_default_world_generator(
        &mut self,
        arg0: impl Into<&'mc String>,
        arg1: impl Into<&'mc String>,
    ) -> Result<crate::generator::ChunkGenerator<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let val_2 = jni::objects::JObject::from(self.jni_ref().new_string(arg1.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getDefaultWorldGenerator",
            "(Ljava/lang/String;Ljava/lang/String;)Lorg/bukkit/generator/ChunkGenerator;",
            &[
                jni::objects::JValueGen::from(&val_1),
                jni::objects::JValueGen::from(&val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::generator::ChunkGenerator::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Returns the name of the plugin.
    /// <p>This should return the bare name of the plugin and should be used for comparison.</p>
    pub fn name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getName", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    /// Gets an embedded resource in this plugin
    pub fn get_resource(
        &mut self,
        arg0: impl Into<&'mc String>,
    ) -> Result<jni::objects::JObject<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(self.jni_ref().new_string(arg0.into()).unwrap());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getResource",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            &[jni::objects::JValueGen::from(&val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.l().unwrap())
    }
    /// Returns the plugin logger associated with this server's logger. The returned logger automatically tags all log messages with the plugin's name.
    pub fn logger(
        &mut self,
    ) -> Result<blackboxmc_java::logging::JavaLogger<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLogger",
            "()Ljava/util/logging/Logger;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_java::logging::JavaLogger::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn on_tab_complete(
        &mut self,
        arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,
        arg1: impl Into<&'mc crate::command::Command<'mc>>,
        arg2: impl Into<&'mc String>,
        arg3: Vec<impl Into<String>>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let res = self.jni_ref().call_method(&self.jni_object(),"onTabComplete","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Ljava/util/List;",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let mut list = blackboxmc_java::JavaList::from_raw(&self.jni_ref(), res.l()?)?;
        let size = list.size()?;
        for i in 0..=size {
            let obj = list.get(i)?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }

    pub fn on_command(
        &mut self,
        arg0: impl Into<&'mc crate::command::CommandSender<'mc>>,
        arg1: impl Into<&'mc crate::command::Command<'mc>>,
        arg2: impl Into<&'mc String>,
        arg3: Vec<impl Into<String>>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_1 = unsafe { jni::objects::JObject::from_raw(arg0.into().jni_object().clone()) };
        let val_2 = unsafe { jni::objects::JObject::from_raw(arg1.into().jni_object().clone()) };
        let val_3 = jni::objects::JObject::from(self.jni_ref().new_string(arg2.into()).unwrap());
        let res = self.jni_ref().call_method(&self.jni_object(),"onCommand","(Lorg/bukkit/command/CommandSender;Lorg/bukkit/command/Command;Ljava/lang/String;Ljava/lang/String;)Z",&[jni::objects::JValueGen::from(&val_1),jni::objects::JValueGen::from(&val_2),jni::objects::JValueGen::from(&val_3)]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
}
impl<'mc> JNIRaw<'mc> for Plugin<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Into<crate::command::TabExecutor<'mc>> for Plugin<'mc> {
    fn into(self) -> crate::command::TabExecutor<'mc> {
        crate::command::TabExecutor::from_raw(&self.jni_ref(), self.1).unwrap()
    }
}
pub mod java;
pub mod messaging;
