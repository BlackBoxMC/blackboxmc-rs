use std::collections::HashSet;

use proc_macro::TokenStream as OriginalTokenStream;
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use rustc_version::{version_meta, Channel};
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, ItemImpl, Receiver, Token,
};

struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Indent
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

/**
    ## extends_blackbox
    Procedural macro that lets you extend BlackBox structs in a Rust-idomatic manner, although the way it is done is not Rust-idomatic at all. It enforces a set of rules on your struct and then creates the proper external facing functions according to the standard that the BlackBoxMC plugin expects.

    ### Caveats
    Java requires an object representation of your struct to be created, and in order for the macro to work, the "new" method has to take a special set of arguments and do something specific. It is implicitly created for you, and to avoid confusion and other conflicts, you lose the ability to make it yourself (however, an alternative macro that doesn't create the `new` method may be created). You can, however, create a method titled `_new`. If you do so, the generated new function will call that one while it is creating the struct, and any arguments that `_new` takes will be added to `new`.

    `__finalize` is also a reserved function name, because it is used in dynamic structs as explained below.

    You have control over whether the overridden functions have `self` (or a immutable/mutable reference to it) as the first parameter or not. This leads to the macro supporting two types of structs:
    * Static structs: Structs in which none of the functions call `self`. You do still have to instantiate the struct via the aformentioned generated `new` method, but none of the pitfalls of dynamic structs apply.
    * Dynamic structs: Struct in which some, or all, of the functions use `self`. This creates many logistical challenges because Java needs to be able to work with a dynamic struct via static functions. The solution is very much not in the name of Rust, but this is a sacrifice to be made: for these structs, the macro will generate a static [MemoryMap](blackbox_rs::macros::memory::MemoryMap) of any instantiated structs that lasts the lifetime of the program, or until Java's GC drops the object that is bound to your struct (a behavior which this macro implements for you), whichever comes first. The static functions will then reference this memory map. There is a lot of room for error here, and if this makes you feel uncomfortable, you should probably use static structs. However, this evil is required for this functionality to actually be useful for many extendable structs.

    *TODO: table of what can be overwritten and what the functions to be extended are*
*/
#[proc_macro_attribute]
pub fn extends_blackbox(
    attr: OriginalTokenStream,
    item: OriginalTokenStream,
) -> OriginalTokenStream {
    match extends_general(
        ExtendType::from(attr.to_string().as_str()),
        attr,
        item.clone(),
    ) {
        Ok(tokens) => return tokens.into(),
        Err(diag) => {
            return format!(
                "{}\n{}",
                item.to_string(),
                diag.iter()
                    .map(|f| f.clone().emit_as_item_tokens().to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .parse()
            .unwrap();
        }
    }
    item
}

enum ExtendType {
    BiomeProvider,
    BlockPopulator,
    ChunkGenerator,
    CommandExecutor,
    ConversationCanceller,
    ConversationPrefix,
    HelpTopic,
    MapRenderer,
    MetadataValue,
    NoiseGenerator,
    PersistentDataType,
    PluginBase,
    Plugin,
    Prompt,
    BukkitRunnable,
    TabCompleter,
    TabExecutor,
    Unknown,
}

impl From<&str> for ExtendType {
    fn from(value: &str) -> Self {
        match value {
            "BiomeProvider" => ExtendType::BiomeProvider,
            "BlockPopulator" => ExtendType::BlockPopulator,
            "ChunkGenerator" => ExtendType::ChunkGenerator,
            "CommandExecutor" => ExtendType::CommandExecutor,
            "ConversationCanceller" => ExtendType::ConversationCanceller,
            "ConversationPrefix" => ExtendType::ConversationPrefix,
            "HelpTopic" => ExtendType::HelpTopic,
            "MapRenderer" => ExtendType::MapRenderer,
            "MetadataValue" => ExtendType::MetadataValue,
            "NoiseGenerator" => ExtendType::NoiseGenerator,
            "PersistentDataType" => ExtendType::PersistentDataType,
            "PluginBase" => ExtendType::PluginBase,
            "Plugin" => ExtendType::Plugin,
            "Prompt" => ExtendType::Prompt,
            "BukkitRunnable" => ExtendType::BukkitRunnable,
            "TabCompleter" => ExtendType::TabCompleter,
            "TabExecutor" => ExtendType::TabExecutor,
            _ => ExtendType::Unknown,
        }
    }
}

impl Into<String> for ExtendType {
    fn into(self) -> String {
        match self {
            ExtendType::BiomeProvider => "BiomeProvider",
            ExtendType::BlockPopulator => "BlockPopulator",
            ExtendType::ChunkGenerator => "ChunkGenerator",
            ExtendType::CommandExecutor => "CommandExecutor",
            ExtendType::ConversationCanceller => "ConversationCanceller",
            ExtendType::ConversationPrefix => "ConversationPrefix",
            ExtendType::HelpTopic => "HelpTopic",
            ExtendType::MapRenderer => "MapRenderer",
            ExtendType::MetadataValue => "MetadataValue",
            ExtendType::NoiseGenerator => "NoiseGenerator",
            ExtendType::PersistentDataType => "PersistentDataType",
            ExtendType::PluginBase => "PluginBase",
            ExtendType::Plugin => "Plugin",
            ExtendType::Prompt => "Prompt",
            ExtendType::BukkitRunnable => "BukkitRunnable",
            ExtendType::TabCompleter => "TabCompleter",
            ExtendType::TabExecutor => "TabExecutor",
            ExtendType::Unknown => unreachable!(),
        }
        .to_string()
    }
}

fn type_name(ty: &syn::Type, reference: bool, mutable: bool, is_array: bool) -> String {
    match ty {
        syn::Type::Path(path) => {
            let mutstr = if mutable { "mut" } else { "" };
            let refstr = if reference { "&" } else { "" };
            format!(
                "{}{} {}",
                refstr,
                mutstr,
                path.path
                    .segments
                    .first()
                    .unwrap()
                    .ident
                    .clone()
                    .to_string()
            )
        }
        syn::Type::Ptr(ptr) => type_name(&*ptr.elem, true, ptr.mutability.is_some(), is_array),
        syn::Type::Reference(rf) => type_name(&*rf.elem, true, rf.mutability.is_some(), is_array),
        syn::Type::Array(ar) => {
            format!(
                "&mut &[{}]",
                type_name(&ar.elem, false, false, is_array).replace(" ", "")
            )
        }
        syn::Type::Slice(ar) => {
            format!(
                "&mut &[{}]",
                type_name(&ar.elem, false, false, is_array).replace(" ", "")
            )
        }
        syn::Type::BareFn(_) => todo!("BareFn"),
        syn::Type::Group(_) => todo!("Group"),
        syn::Type::ImplTrait(_) => todo!("ImplTrait"),
        syn::Type::Infer(_) => todo!("Infer"),
        syn::Type::Macro(_) => todo!("Macro"),
        syn::Type::Never(_) => todo!("Never"),
        syn::Type::Paren(_) => todo!("Paren"),
        syn::Type::Tuple(_) => todo!("Tuple"),
        syn::Type::TraitObject(_) => todo!("TraitObj"),

        syn::Type::Verbatim(_) => todo!("Verbatim"),
        _ => todo!(),
    }
}

fn resolve_self_name(s: &Receiver) -> String {
    let mutstr = if s.mutability.is_some() { "mut" } else { "" };
    let refstr = match &s.reference {
        Some(a) => match &a.1 {
            Some(l) => {
                format!("&'{}", l.ident)
            }
            None => format!("&"),
        },
        None => {
            format!("")
        }
    };
    format!("{}{} self", refstr.as_str().replace(" ", ""), mutstr)
}

fn extends_general(
    ty: ExtendType,
    attr_: OriginalTokenStream,
    item_: OriginalTokenStream,
) -> Result<TokenStream, Vec<Diagnostic>> {
    let basis = &item_.to_string();

    let attr: TokenStream = attr_.into();
    let item = match syn::parse::<ItemImpl>(item_) {
        Ok(data) => data,
        Err(err) => {
            return Ok(OriginalTokenStream::from(err.to_compile_error()).into());
        }
    };

    let mut errors: Vec<(Span, String)> = vec![];
    let mut notes: Vec<(Span, String)> = vec![];

    let abstract_functions = ExtendRequirements::from_type(&ty).abstract_functions;
    let abstract_functions_with_args =
        ExtendRequirements::from_type(&ty).abstract_functions_with_args;

    let struct_name = type_name(&*item.self_ty, false, false, false);

    // Do a sanity check on the impl.
    function_sanity_check(
        ty,
        attr,
        item,
        &abstract_functions_with_args,
        &mut errors,
        &mut notes,
    );

    // Generate the real deal.

    // ...

    if errors.len() >= 1 || notes.len() >= 1 {
        let mut final_errors: Vec<Diagnostic> = vec![];
        for (span, error) in errors {
            final_errors.push(span.error(error))
        }
        for (span, note) in notes {
            final_errors.push(span.note(note))
        }
        return Err(final_errors);
    }
    Ok(basis.parse().unwrap())
}

fn function_sanity_check(
    ty: ExtendType,
    attr: TokenStream,
    item: ItemImpl,
    abstract_functions_with_args: &Vec<(&str, Vec<&str>, &str)>,
    errors: &mut Vec<(Span, String)>,
    notes: &mut Vec<(Span, String)>,
) {
    let mut corr_functions = vec![];

    for item in item.items.clone() {
        match item {
            syn::ImplItem::Macro(_) => todo!(),
            syn::ImplItem::Const(_) => todo!(),
            syn::ImplItem::Fn(f) => {
                let mut self_sig = String::new();
                let name = f.sig.ident.to_string();
                if validate_fn_for_required(&ty, name.as_str()) {
                    // get the self reciever
                    f.sig.inputs.iter().for_each(|f| {
                        if let syn::FnArg::Receiver(r) = f {
                            self_sig = resolve_self_name(r).clone();
                        }
                    });
                    // It has the name. Does it have the correct amount of arguments?
                    let args = f
                        .sig
                        .inputs
                        .iter()
                        .filter(|f| {
                            if let syn::FnArg::Typed(_) = f {
                                true
                            } else {
                                false
                            }
                        })
                        .map(|f| {
                            if let syn::FnArg::Typed(f) = f {
                                (f.span(), type_name(&*f.ty, false, false, false))
                            } else {
                                unreachable!()
                            }
                        })
                        .collect::<Vec<(Span, String)>>();

                    let aargs = function_args_from_extend_requirement_filter_name(
                        name.clone(),
                        &abstract_functions_with_args,
                    );
                    let mut aargs_disp = vec![self_sig.clone()];
                    aargs_disp.append(&mut aargs.clone());
                    let mut args_disp = vec![self_sig.clone()];
                    args_disp.append(&mut args.iter().map(|f| f.1.clone()).collect());

                    if aargs.len() != args.len() {
                        errors.push((
                            f.sig.span(),
                            format!(
                                "{} {} \n expected `{}`. got `{}`",
                                aargs.len(),
                                args.len(),
                                aargs_disp.join(", ").replace("  ", " ").replace("& ", "&"),
                                args_disp.join(", ").replace("  ", " ").replace("& ", "&"),
                            ),
                        ));
                    } else {
                        // Are the functions all the correct type?
                        for i in 0..aargs.len() {
                            let aarg = aargs.get(i).unwrap();
                            let arg = args.get(i).unwrap();
                            if aarg != &arg.1 {
                                errors.push((
                                    arg.0,
                                    format!("expected `{}`.. got `{}`", aarg, arg.1),
                                ));
                            }
                        }
                    }

                    let return_arg =
                        function_return_arg(name.clone(), &abstract_functions_with_args);
                    match f.sig.output {
                        syn::ReturnType::Default => {
                            if return_arg != "()" {
                                errors.push((
                                    f.sig.span(),
                                    format!("function was expected to return {}", return_arg),
                                ));
                            }
                        }
                        syn::ReturnType::Type(_, ty) => {
                            let tyname = type_name(&*ty, false, false, false).replace(" ", "");
                            if tyname != return_arg {
                                errors.push((
                                    ty.span(),
                                    format!("expected {}, got {}", return_arg, tyname),
                                ));
                            }
                        }
                    };
                    corr_functions.push(name);
                }
            }
            syn::ImplItem::Type(_) => todo!(),

            syn::ImplItem::Verbatim(_) => todo!(),
            _ => todo!(),
        }
    }
    let functions: String =
        function_signature_from_extend_requirement(&corr_functions, &abstract_functions_with_args);
    if functions.len() >= 1 {
        errors.push((
            item.span(),
            format!(
                "not all equivalant functions implemented, missing: \n```\n\t{}\n```",
                functions
            ),
        ));
        notes.push((item.span(), format!("{}", functions)));
    }
}

fn function_signature_from_extend_requirement(
    corr_functions: &Vec<String>,
    abstract_functions_with_args: &Vec<(&str, Vec<&str>, &str)>,
) -> String {
    abstract_functions_with_args
        .iter()
        .filter(|f| !corr_functions.contains(&f.0.to_string()))
        .map(|f| {
            let name = format!("&mut {}", f.0.to_string().to_string());
            let args = f.1.join(",").to_string();
            format!("fn {}({})", name, args)
        })
        .collect::<Vec<String>>()
        .join(",\n\t")
        .to_string()
}

fn function_return_arg(
    name: String,
    abstract_functions_with_args: &Vec<(&str, Vec<&str>, &str)>,
) -> String {
    abstract_functions_with_args
        .iter()
        .filter(|f| f.0.to_string() == name)
        .map(|f| f.2.to_string())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .clone()
}

fn function_args_from_extend_requirement_filter_name(
    name: String,
    abstract_functions_with_args: &Vec<(&str, Vec<&str>, &str)>,
) -> Vec<String> {
    abstract_functions_with_args
        .iter()
        .filter(|f| f.0.to_string() == name)
        .flat_map(|f| f.1.clone())
        .map(|f| format!("&mut {}", f.to_string()))
        .collect::<Vec<String>>()
}

struct ExtendRequirements<'a> {
    abstract_functions: Vec<&'a str>,
    abstract_functions_with_args: Vec<(&'a str, Vec<&'a str>, &'a str)>,
}
impl<'a> ExtendRequirements<'a> {
    fn from_type(ty: &ExtendType) -> Self {
        match ty {
            ExtendType::BiomeProvider => Self::biome_provder(),
            ExtendType::BlockPopulator => Self::block_populator(),
            ExtendType::ChunkGenerator => Self::chunk_generator(),
            ExtendType::CommandExecutor => Self::command_executor(),
            ExtendType::ConversationCanceller => Self::conversation_canceller(),
            ExtendType::ConversationPrefix => Self::conversation_prefix(),
            ExtendType::HelpTopic => Self::help_topic(),
            ExtendType::MapRenderer => Self::map_renderer(),
            ExtendType::MetadataValue => Self::metadata_value(),
            ExtendType::NoiseGenerator => Self::noise_generator(),
            ExtendType::PersistentDataType => Self::persistent_data_type(),
            ExtendType::PluginBase => Self::plugin_base(),
            ExtendType::Plugin => Self::plugin(),
            ExtendType::Prompt => Self::prompt(),
            ExtendType::BukkitRunnable => Self::bukkit_runnable(),
            ExtendType::TabCompleter => Self::tab_completer(),
            ExtendType::TabExecutor => Self::tab_completer(),
            ExtendType::Unknown => unreachable!(),
        }
    }
    fn biome_provder() -> Self {
        Self {
            abstract_functions: vec!["get_biome", "get_biomes"],
            abstract_functions_with_args: vec![
                (
                    "get_biome",
                    vec!["Plugin", "WorldInfo", "i32", "i32", "i32"],
                    "Biome",
                ),
                ("get_biomes", vec!["Plugin", "WorldInfo"], "Vec<Biome>"),
            ],
        }
    }
    fn command_executor() -> Self {
        Self {
            abstract_functions: vec!["on_command"],
            abstract_functions_with_args: vec![(
                "on_command",
                vec!["Plugin", "CommandSender", "Command", "String", "&[String]"],
                "bool",
            )],
        }
    }
    /*fn config_serializable() -> Self {
        Self {
            abstract_functions: vec!["serialize"],
            abstract_functions_with_args: vec![("serialize", vec!["Plugin"])],
        }
    }*/
    fn conversation_canceller() -> Self {
        Self {
            abstract_functions: vec!["cancel_based_on_input", "clone", "set_conversation"],
            abstract_functions_with_args: vec![
                (
                    "cancel_based_on_input",
                    vec!["Plugin", "ConversationContext", "String"],
                    "bool",
                ),
                ("clone", vec!["Plugin"], "ConversationCanceller"),
                ("set_conversation", vec!["Plugin", "Conversation"], "()"),
            ],
        }
    }
    fn conversation_prefix() -> Self {
        Self {
            abstract_functions: vec!["get_prefix"],
            abstract_functions_with_args: vec![(
                "get_prefix",
                vec!["Plugin", "ConvesrationContext"],
                "String",
            )],
        }
    }
    fn help_topic() -> Self {
        Self {
            abstract_functions: vec!["can_see"],
            abstract_functions_with_args: vec![(
                "can_see",
                vec!["Plugin", "CommandSender"],
                "bool",
            )],
        }
    }
    fn map_renderer() -> Self {
        Self {
            abstract_functions: vec!["render"],
            abstract_functions_with_args: vec![(
                "render",
                vec!["Plugin", "MapView", "MapCanvas", "Player"],
                "void",
            )],
        }
    }
    fn metadata_value() -> Self {
        Self {
            abstract_functions: vec![
                "as_boolean",
                "as_byte",
                "as_double",
                "as_float",
                "as_int",
                "as_long",
                "as_short",
                "as_string",
                "get_owning_plugin",
                "invalidate",
                "value",
            ],
            abstract_functions_with_args: vec![
                ("as_boolean", vec!["Plugin"], "boolean"),
                ("as_byte", vec!["Plugin"], "i8"),
                ("as_double", vec!["Plugin"], "f64"),
                ("as_float", vec!["Plugin"], "f32"),
                ("as_int", vec!["Plugin"], "i32"),
                ("as_long", vec!["Plugin"], "i64"),
                ("as_short", vec!["Plugin"], "i16"),
                ("as_string", vec!["Plugin"], "String"),
                ("get_owning_plugin", vec!["Plugin"], "Plugin"),
                ("invalidate", vec!["Plugin"], "()"),
                ("value", vec!["Plugin"], "jni::objects::Object"),
            ],
        }
    }
    fn noise_generator() -> Self {
        Self {
            abstract_functions: vec!["noise"],
            abstract_functions_with_args: vec![(
                "noise",
                vec!["Plugin", "f64", "f64", "f64"],
                "f64",
            )],
        }
    }

    /*fn plugin_loader() -> Self {
        Self {
            abstract_functions: vec![
                "create_registered_listeners",
                "disable_plugin",
                "enable_plugin",
                "get_plugin_description",
                "get_plugin_file_filters",
                "load_plugin",
            ],
            abstract_functions_with_args: vec![
                ("create_registered_listeners", vec!["Plugin", "Listener"]),
                ("disable_plugin", vec!["Plugin"]),
                ("enable_plugin", vec!["Plugin"]),
                ("get_plugin_description", vec!["Plugin", "File"]),
                ("get_plugin_file_filters", vec!["Plugin"]),
                ("load_plugin", vec!["Plugin", "File"]),
            ],
        }
    }*/
    fn plugin() -> Self {
        Self {
            abstract_functions: vec![
                "get_config",
                "get_data_folder",
                "get_default_biome_provider",
                "get_default_world_generator",
                "get_description",
                "get_logger",
                "get_name",
                "get_plugin_loader",
                "get_resource",
                "get_server",
                "is_enabled",
                "is_naggable",
                "on_disable",
                "on_enable",
                "on_load",
                "reload_config",
                "save_config",
                "save_default_config",
                "save_resource",
                "set_naggable",
            ],
            abstract_functions_with_args: vec![
                ("get_config", vec!["Plugin"], "FileConfiguration"),
                ("get_data_folder", vec!["Plugin"], "File"),
                (
                    "get_default_biome_provider",
                    vec!["Plugin", "String", "String"],
                    "BiomeProvider",
                ),
                (
                    "get_default_world_generator",
                    vec!["Plugin", "String", "String"],
                    "ChunkGenerator",
                ),
                ("get_description", vec!["Plugin"], "PluginDescriptionFile"),
                ("get_logger", vec!["Plugin"], "Logger"),
                ("get_name", vec!["Plugin"], "String"),
                ("get_plugin_loader", vec!["Plugin"], "PluginLoader"),
                ("get_resource", vec!["Plugin", "String"], "InputStream"),
                ("get_server", vec!["Plugin"], "Server"),
                ("is_enabled", vec!["Plugin"], "bool"),
                ("is_naggable", vec!["Plugin"], "bool"),
                ("on_disable", vec!["Plugin"], "()"),
                ("on_enable", vec!["Plugin"], "()"),
                ("on_load", vec!["Plugin"], "()"),
                ("reload_config", vec!["Plugin"], "()"),
                ("save_config", vec!["Plugin"], "()"),
                ("save_default_config", vec!["Plugin"], "()"),
                ("save_resource", vec!["Plugin", "String", "bool"], "()"),
                ("set_naggable", vec!["Plugin", "bool"], "()"),
            ],
        }
    }
    fn prompt() -> Self {
        Self {
            abstract_functions: vec!["accept_input", "blocks_for_input", "get_prompt_text"],
            abstract_functions_with_args: vec![
                (
                    "accept_input",
                    vec!["Plugin", "ConversationContext", "String"],
                    "Prompt",
                ),
                (
                    "blocks_for_input",
                    vec!["Plugin", "ConversationContext"],
                    "bool",
                ),
                (
                    "get_prompt_text",
                    vec!["Plugin", "ConversationContext"],
                    "String",
                ),
            ],
        }
    }
    fn bukkit_runnable() -> Self {
        Self {
            abstract_functions: vec!["run"],
            abstract_functions_with_args: vec![("run", vec!["Plugin"], "()")],
        }
    }
    fn tab_completer() -> Self {
        Self {
            abstract_functions: vec!["on_tab_complete"],
            abstract_functions_with_args: vec![(
                "on_tab_complete",
                vec!["Plugin", "CommandSender", "Command", "String", "String"],
                "Vec<String>",
            )],
        }
    }
    fn tab_executor() -> Self {
        Self::tab_completer()
    }

    fn block_populator() -> Self {
        Self {
            abstract_functions: vec![],
            abstract_functions_with_args: vec![],
        }
    }
    fn chunk_generator() -> Self {
        Self {
            abstract_functions: vec![],
            abstract_functions_with_args: vec![],
        }
    }
    fn persistent_data_type() -> Self {
        Self {
            abstract_functions: vec![],
            abstract_functions_with_args: vec![],
        }
    }
    fn plugin_base() -> Self {
        Self {
            abstract_functions: vec![],
            abstract_functions_with_args: vec![],
        }
    }
}

fn validate_fn_for_required(ty: &ExtendType, name: &str) -> bool {
    if ExtendRequirements::from_type(&ty)
        .abstract_functions
        .contains(&name)
    {
        true
    } else {
        false
    }
}
