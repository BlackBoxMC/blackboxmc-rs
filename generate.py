#!/usr/bin/python
import json
import os
import pathlib
import shutil
import re
import multiprocessing

f = open("./spigot.json")
libraries = json.loads(f.read())

# files to write to disk
file_cache = {}

# rust's reserved words; we use this to rename any functions
reserved_words = ["as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
                  "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
                  "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield"]

library_resolves = {
    "net.md_5": "blackboxmc-rs-bungee",
    "org.bukkit": "blackboxmc-rs-bukkit",
    "java.util": "blackboxmc-rs-java/src/util",
    "java.lang": "blackboxmc-rs-java/src/lang",
}

parsed_classes = {}
excluded_classes = [
    "org.bukkit.plugin.SimplePluginManager",    # uses stuff that isn't being generated due to that java bug and i don't want to write an entire class binding for something that's getting deprecated anyways.
    "java.lang.JavaThread", "java.lang.JavaIterable", "java.lang.JavaRunnable", "java.lang.JavaCharSequence", "java.util.regex.JavaMatcher", "java.util.JavaObservable", "java.util.JavaFormatter", "java.lang.JavaException", "java.util.JavaResourceBundle", "java.lang.JavaThrowable", "java.lang.JavaCloneable", "java.lang.JavaComparable", "java.lang.JavaClass", "java.lang.JavaStringBuffer","java.lang.JavaStringBuilder",

    "java.lang.constant",
    "java.lang.annotation",
    "java.lang.Number",
    "java.lang.invoke",

    "java.util.concurrent",                     # i want to lessen my workload and binding this is going to be pointless since your only option of multithreading is through BukkitRunnables.
    "java.util.stream",                         # unneeded when java's iterator is bound.

    "net.md_5.bungee.chat.TranslationRegistry$TranslationProvider",
    "org.bukkit.plugin.SimpleServicesManager",

    "java.util.JavaLocale$Category", "java.util.JavaLocale$FilteringMode",

    # dear sweet baby jesus i just want to be done with this and i will remove half of the java.util
    # package if it means fucking getting this done.
    "java.util.PropertyResourceBundle", "java.util.Currency", "java.util.EnumMap", "java.util.Spliterators",
                        "java.util.Spliterators$AbstractIntSpliterator",
                        "java.util.Spliterators$AbstractLongSpliterator", "java.util.Spliterators$AbstractSpliterator",
                        "java.util.SplittableRandom", "java.util.AbstractMap$SimpleImmutableEntry", "java.util.Arrays", "java.util.GregorianCalendar",
    "java.util.NavigableMap", "java.util.NavigableSet",
    "java.util.Base64$Decoder", "java.util.Base64$Encoder", "java.util.BitSet",
                        "java.util.Calendar", "java.util.Calendar$Builder", "java.util.Collections",
                        "java.util.Dictionary",
                        "java.util.DoubleSummaryStatistics",
                        "java.util.EventListenerProxy", "java.util.EventObject", "java.util.FormattableFlags",
                        "java.util.Formatter", "java.util.GregorianCalendar", "java.util.IntSummaryStatistics",
                        "java.util.ListResourceBundle", "java.util.LongSummaryStatistics", "java.util.Objects",
                        "java.util.Observable",
                        "java.util.PriorityQueue",
                        "java.util.Properties", "java.util.PropertyPermission",
                        "java.util.ResourceBundle", "java.util.ResourceBundle$Control",
                        "java.util.Scanner", "java.util.ServiceLoader", "java.util.SimpleTimeZone",
                        "java.util.Stack", "java.util.StringJoiner",
                        "java.util.StringTokenizer", "java.util.Timer", "java.util.TimerTask", "java.util.TimeZone", "java.util.Spliterator", "java.util.Spliterator$OfDouble", "java.util.Spliterator$OfInt",
                        "java.util.Spliterator$OfLong", "java.util.Spliterator$OfPrimitive", "java.lang.StackTraceElement", "java.lang.Throwable",
                        "java.util.logging.LoggingMXBean", "java.util.logging.FileHandler",
                        "java.util.logging.LogManager",

    # tempoerary
    "java.util.JavaSortedMap", "java.util.JavaSortedSet"
]

interface_names = []
filled_once = []
bindings = {}
omitted_classes = []
enums = []

def library_name_no_extra_paths(libname):
    return re.sub("\/src\/(.*)","",libname)

def library_name_format(crate_name,library):
    l1 = library_resolves[crate_name]
    l2 = library_resolves[library]
    st1 = re.sub("\/src\/(.*)","::\\1",l1.replace("-","_").replace("_rs_","_"))
    st2 = re.sub("\/src\/(.*)","",l2.replace("-","_").replace("_rs_","_"))
    if st2.replace("::","") in st1:
        st1 = st1.replace(st2,"crate")
    return st1

def library_name_format_to_crate(libname):
    reg = re.search("\/src\/(.*)",libname)
    if reg is not None:
        return "crate::"+reg.group(1)
    else:
        return "crate"

def in_excluded_classes(cl):
    if cl in excluded_classes:
        return True
    for c in excluded_classes:
        if cl.startswith(c):
            return True
    return False

def mod_rs_folder_populate(dir):
    files = os.listdir(dir)
    f = open(dir+os.sep+"mod.rs", "a")
    f2 = open(dir+os.sep+"mod.rs", "r")
    for file in files:
        filename = str(file).lower()

        suffix = ""
        if filename.endswith(".rs"):
            suffix = ".rs"
        if filename.replace(".rs", "") in reserved_words:
            shutil.move(os.path.join(dir, filename+suffix),
                        os.path.join(dir, "mod_"+filename+suffix))
            filename = "mod_"+filename+suffix

        file3 = os.path.join(dir, filename)
        if os.path.isdir(file3):
            to_write = "pub mod "+filename+";\n"
            # check if the file already has this line because i don't even- what?
            what = list(filter(lambda f: "pub mod" in f, f2.readlines()))
            if to_write not in what:
                f.write(to_write)
                mod_rs_folder_populate(file3)
    f.close()
    f2.close()

def camel_case_to_snake_case(string):
    return re.sub(r"([a-z])([A-Z])", r"\1_\2",
                    string).lower()

def snake_case_to_camel_case(string):
    return re.sub(r"_([a-z])", lambda f: f.group(1).upper(),
                    string.lower()).replace("_","")

def func_signature_format(ty, increment, returning, options_start_at=-1):
    thing_start = ""
    thing = ""
    ty["type_name_resolved"] = ty["type_name_resolved"].replace("<T>","")
    internal_do_into = ty["type_name_resolved"].startswith("crate") or ty["type_name_resolved"].startswith("bukkit") or ty["type_name_resolved"].startswith("blackbox")
    is_string = ty["type_name_resolved"] == "String"
    internal = internal_do_into or ty["type_name_resolved"].startswith("jni")
    generic_letters = []
    generic_args = []
    if(ty["type_name_lhand"] == "" and not returning):
        thing_start += ty["type_name_resolved"]
    else:
        if not returning:
            thing_start += ty["type_name_lhand"]+": "

        if(options_start_at > -1):
            if increment >= options_start_at:
                thing += "std::option::Option<"

        if(ty["type_name_resolved"].startswith("&dyn")):
            old = ty["type_name_resolved"]
            letter = chr(ord('a')+increment).upper()
            ty["type_name_resolved"] = letter
            generic_letters.append(letter)
            generic_args.append(letter+": blackboxmc_general::JNIRaw<'mc> + "+old.replace("&dyn","")+"<'mc>")

        if ty["is_array"]:
            thing += "Vec<"
            if (internal_do_into or is_string) and not returning:
                thing += "impl Into<"
            thing += ty["type_name_resolved"]
            if(internal and "'mc" not in ty["type_name_resolved"]):
                thing += "<'mc>"
            if (internal_do_into or is_string) and not returning:
                thing += ">"
            thing += ">"
        else:
            if (internal_do_into or is_string) and not returning:
                thing += "impl Into<"
            if len(ty["generics"]) >= 1:
                thing += ty["type_name_alone"]+"<"
                j = []
                for g in ty["generics"]:
                    g = g["type_name_resolved"]
                    if len(g) >= 2:
                        if g.startswith("crate") or g.startswith("bukkit") or g.startswith("jni") or g.startswith("blackbox") or g == "String":
                            k = ""
                            if not returning and not g.startswith("jni"):
                                k += "impl Into<"
                            if "'mc" not in g and g != "String":
                                k += g+"<'mc>"
                            else:
                                k += g
                            if not returning and not g.startswith("jni"):
                                k += ">"
                            j.append(k)
                        else:
                            j.append(g)
                    else:
                        return None
                thing += ",".join(j)
                thing += ">"
            else:
                thing += ty["type_name_resolved"]
                if(internal):
                    thing += "<'mc>"
            if (internal_do_into or is_string) and not returning:
                thing += ">"
        if(options_start_at != -1):
            if increment >= options_start_at:
                thing += ">"

    name_only = ""
    return {
        "result": thing_start+thing,
        "result_name_only": thing_start.split(":")[0],
        "result_type_only": thing,
        "generic_letters": generic_letters,
        "generic_args": generic_args,
    }

def argument_name_format(objects):
    parts = objects[1].split(".")
    name = parts[len(parts) - 1].replace("[]", "s").replace("$", "")
    return re.sub(r"([a-z])([A-Z])", r"\1_\2", name).lower()

def code_format(type, prefix, n, var_prefix="val", arg="", class_name="", options_start_at=-1, no_sig=False):
    if arg == "":
        if "type_name_lhand" in type:
            arg = type["type_name_lhand"]
        else:
            arg = "arg"+str(n)

    ty = type["type_name_resolved"].replace("&dyn ","")

    res = []

    do_option = n >= options_start_at and options_start_at != -1


    let = java_letter_from_rust(type["type_name_original"])
    if do_option:
        res.append("if let Some(a) = "+arg+" {")
        if not no_sig:
            res.append("sig += \""+let+"\";")
        new_arg = "a"
    else:
        new_arg = arg
        if options_start_at != -1:
            res.append("sig += \""+let+"\";")

    if(ty.startswith("crate") or ty.startswith("blackbox")): # for internal types...
        if "JavaEnum" in ty:
            return None

        # get the original object.
        fcall = ".into().jni_object().clone()"
        if type["type_name_resolved"].startswith("&dyn"):
            fcall = ".jni_object().clone()"

        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw("+new_arg+fcall+")});")
    else:
        match(type["type_name_alone"].replace("Java","")):
            case "bool" | "i8" | "char" | "f64" | "f32" | "i32" | "i64" | "i16" | "u16":
                v = java_type_from_rust(type)
                match(type["type_name_original"]):
                    case "boolean":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Bool("+new_arg+".into());")
                    case "byte":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Byte("+new_arg+".into());")
                    case "char":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Char("+new_arg+".into());")
                    case "double":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Double("+new_arg+".into());")
                    case "float":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Float("+new_arg+".into());")
                    case "int":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Int("+new_arg+".into());")
                    case "long":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Long("+new_arg+".into());")
                    case "short":
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Short("+new_arg+".into());")
                    case _:
                        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+prefix+".new_object(\""+type["type_name_original"].replace(".","/").replace("Java","")+"\", \""+v["function_signature"]+"\", vec!["+new_arg+".into()])?);")
            case "String":
                res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(jni::objects::JObject::from("+prefix+".new_string("+new_arg+".into())?));")
            case "jni::objects::JObject":
                res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+new_arg+");")
            case "jni::objects::JClass":
                res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+new_arg+".into());")
            case "Vec":
                match type["type_name_original"]:
                    # TODO: convert this to use the new java.util bindings instad.
                    case "java.util.List":
                        c = [
                            "let raw_"+var_prefix+"_"+str(n)+" = "+prefix+".new_object(\"java/util/ArrayList\", \"()V\", vec![])?;",
                            "for v in "+new_arg+"{"
                        ]

                        if 0 not in type["generics"]:
                            return None

                        t1 = java_type_from_rust(type["generics"][0])["class_name"]

                        co = code_format({
                            "type_name_resolved": type["generics"][0]["type_name_resolved"],
                            "type_name_lhand": "v",
                            "is_array": type["is_array"],
                            "is_interface": type["is_interface"],
                            "type_name_original": t1,
                            "type_name_alone": type["generics"][0]["type_name_resolved"],
                            "generics": type["generics"],
                            "package_name": type["package_name"],
                            "options_start_at": options_start_at,
                            "usage_unsafe": False,
                        }, prefix, 0, "map_val", "v", options_start_at, no_sig=True)
                        if co is None:
                            return None
                        c.append("\n\t\t".join(co))

                        c.append(
                            prefix+".call_method("+
                                "&raw_"+var_prefix+"_"+str(n)+","+
                                "\"add\","
                                "\"(L"+t1+")V\","+
                                "vec![jni::objects::JValueGen::from(map_val_0)]"
                            ")?;"
                        )

                        c.append("};")

                        c.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(raw_"+var_prefix+"_"+str(n)+");")

                        res += c

                    case _:
                        print("Unhandled map type:\t\t"+type["type_name_original"],"\t\t",type["type_name_original"])
                        return None
            case _:
                print("Untranslated argument:\t\t"+type["type_name_alone"],"\t\t",type["type_name_original"])
                return None

    if options_start_at != -1:
        if not no_sig:
            res.append("args.push("+var_prefix+"_"+str(n)+");")
    if do_option:
        res.append("}")

    return res

def return_format_one_liner(val, var_name):
    match val:
        case "()": return "()"
        case "u16": return var_name+".c()?"
        case "i8": return var_name+".b()?"
        case "i16": return var_name+".s()?"
        case "bool": return var_name+".z()?"
        case "i32": return var_name+".i()?"
        case "i64": return var_name+".j()?"
        case "f32": return var_name+".f()?"
        case "f64": return var_name+".d()?"

def return_format(return_group, prefix, static, method, obj_call, func_signature, types, is_trait, options_start_at, is_constructor, nullable, library):
    if return_group["is_array"]:
        return None
    else:
        end_line = "?"
    code = []
    if options_start_at != -1:
        arr = "args"
    else:
        arr = "vec!["+",".join(types)+"]"
    if static:
        code.append("let cls = jni.find_class(\""+return_group["type_name_original"].replace(".","/").replace("Java","")+"\"); let cls = jni.translate_error_with_class(cls)"+end_line+";")
        if is_constructor:
            code.append("let res = "+prefix+".new_object(cls,")
        else:
            code.append("let res = "+prefix+".call_static_method(cls,\""+method["original_name"]+"\",")

        code.append("sig.as_str(),"+arr+");")

        if return_group["type_name_resolved"] != "()":
            code.append("let res = ")
        if is_constructor:
            code.append("jni.translate_error_no_gen(res)"+end_line+";")
        else:
            code.append("jni.translate_error(res)"+end_line+";")
    else:
        code.append(
            "let res = "+prefix+".call_method("+
                    "&"+obj_call+","+
                    "\""+method["original_name"]+"\",sig.as_str(),"+arr+");")
        if return_group["type_name_resolved"] != "()":
            code.append("let res = ")
        code.append(prefix+".translate_error(res)?;")

    # check if the returning object isn't a primitive.
    match return_group["type_name_resolved"]:
        case "()" | "u16" | "i8" | "i16" | "bool" | "i32" | "i64" | "f32" | "f64":
            nop = 0
        case _:
            if nullable:
                code.append("if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}")

    if static:
        val_1 = "jni"
        if is_constructor:
            val_2 = "res"
        else:
            val_2 = "obj"
    else:
        if is_trait:
            val_1 = "self.jni_ref()"
        else:
            val_1 = "self.0"
        val_2 = "unsafe { jni::objects::JObject::from_raw(res.l()"+end_line+".clone()) }"

    # primitive translation should not be done inside the java.lang bindings.
    skip_primitives = False
    if "lang" in library:
        if is_constructor:
            skip_primitives = True
        else:
            skip_primitives = False
    if not skip_primitives:
        match return_group["type_name_resolved"]:
            case "()" | "u16" | "i8" | "i16" | "bool" | "i32" | "i64" | "f32" | "f64":
                code.append("Ok(")
                if nullable:
                    code.append("Some(")
                code.append(return_format_one_liner(return_group["type_name_resolved"],"res"))
                if nullable:
                    code.append(")")
                code.append(")")
                return "\n".join(code)
            case "String":
                code.append("Ok(")
                if nullable:
                    code.append("Some(")
                code.append(prefix+
                            ".get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })"+end_line+
                            ".to_string_lossy()"+
                            ".to_string()"
                )
                if nullable:
                    code.append(")")
                code.append(")")
                return "\n".join(code)
    match return_group["type_name_resolved"]:
        case "()":
            code.append("Ok(")
            if nullable:
                code.append("Some(")
            code.append(return_format_one_liner(return_group["type_name_resolved"],"res"))
            if nullable:
                code.append(")")
            code.append(")")
        case "jni::objects::JObject":
            code.append("Ok(")
            if nullable:
                code.append("Some(")
            if is_constructor:
                code.append("res")
            else:
                code.append("res.l()?")
            if nullable:
                code.append(")")
            code.append(")")
        case "jni::objects::JClass":
            code.append("Ok(")
            if nullable:
                code.append("Some(")
            code.append("unsafe {"+
                        "jni::objects::JClass::from_raw(res.as_jni().l)"+
                        "}")
            if nullable:
                code.append(")")
            code.append(")")
        case "(u8, u8, u8)":
            code.append("let r = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getRed\", \"(V)I\", vec![]);"+
                        "            ;let r = "+prefix+".translate_error(r)?.i()? as u8;"+
                        "let g = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getGreen\", \"(V)I\", vec![])"+
                        "            ; let g = "+prefix+".translate_error(g)?.i()? as u8;"+
                        "let b = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getBlue\", \"(V)I\", vec![])"+
                        "            ; let b = "+prefix+".translate_error(b)?.i()? as u8;"+
                        "Ok(")

            if nullable:
                code.append("Some(")
            code.append("(r, g, b)")
            if nullable:
                code.append(")")
            code.append(")")
        case "Vec":
            code.append("let mut new_vec = Vec::new();")
            normally = ""
            gen = return_group["generics"][0]["type_name_resolved"]
            match return_group["type_name_original"]:
                case "java.util.Collection":
                    code.append("let col = blackboxmc_java::util::JavaCollection::from_raw(&"+
                                                prefix+",res.l()"+end_line+")"+end_line+";"+
                                "let iter = col.iterator()"+end_line+";")
                case "java.util.List":
                    code.append("let list = blackboxmc_java::util::JavaList::from_raw(&"+val_1+", res.l()"+end_line+")"+end_line+";"+
                                "let iter = list.iterator()"+end_line+";")
                case _:
                    print("Unhandled map type as return type:\t\t"+return_group["type_name_original"])
                    return None
            code.append("while iter.has_next()"+end_line+" {"+
                "            let obj = iter.next()"+end_line+";")
            match gen:
                case "()" | "u16" | "i8" | "i16" | "bool" | "i32" | "i64" | "f32" | "f64":
                    return None
                    #code.append("new_vec.push("+return_format_one_liner(gen,"obj")+")")
                case "jni::objects::JObject":
                    code.append("new_vec.push(obj);")
                case "jni::objects::JClass":
                    code.append("new_vec.push(unsafe {"+
                        "jni::objects::JClass::from_raw(*obj)"+
                        "})")
                case "String":
                    code.append("new_vec.push("+prefix+
                        ".get_string(unsafe { &jni::objects::JString::from_raw(*obj) })"+end_line+
                        ".to_string_lossy()"+
                        ".to_string());")
                case _:
                    code.append("new_vec.push("+gen+"::from_raw(&"+val_1+",obj,)"+end_line+");")
            code.append("};Ok(")
            if nullable:
                code.append("Some(")
            code.append("new_vec")
            if nullable:
                code.append(")")
            code.append(")")
        case _:
            if return_group["type_name_resolved"] == "blackboxmc_java::lang::JavaEnum":
                return_group["type_name_resolved"] = "Self"
            if return_group["type_name_resolved"].startswith("&dyn "):
                what = return_group["type_name_alone"].split("::")
                name = what[len(what)-1]
                what.pop()
                return_group["type_name_resolved"] = "::".join(what)+name
            #if return_group["is_array"]:
            #    code.append("let arr = &Into::<jni::objects::JObjectArray>::into(unsafe {"+
            #                "    jni::objects::JObject::from_raw(res.as_jni().l)"+
            #                "});"+
            #                "let num = "+prefix+".get_array_length(arr)"+end_line+";"+
            #                "let mut vec = (0..num)"+
            #                "    .map(|i| {")

            if static and not is_constructor:
                code.append("let obj = res.l()"+end_line+";")

            if return_group["type_name_original"] in enums:
                code.append("let raw_obj = "+val_2+";let variant = "+val_1+".call_method(&raw_obj, \"toString\", \"()Ljava/lang/String;\", vec![]); let variant = "+prefix+".translate_error(variant)"+end_line+";"+
                            "let variant_str = "+val_1+""+
                            "    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })"+end_line+
                            "    .to_string_lossy()"+
                            "    .to_string();")
                val_2 = "raw_obj"

            if nullable:
                code.append("Ok(Some(")
            code.append(return_group["type_name_resolved"]+"::from_raw(&"+
                                                prefix+","+
                                                val_2)
            #if return_group["type_name_original"] in enums:
            #    code.append(", "+return_group["type_name_resolved"]+"::from_string(variant_str).ok_or(eyre::eyre!(\"String gaven for variant was invalid\"))"+end_line)
            if nullable:
                code.append(")"+end_line+")")
            code.append(")")
            #if return_group["is_array"]:
            #    return_val = "Box::leak(Box::new(vec))"
            #    code.append("})"+
            #                ".collect::<Vec<"+return_group["type_name_resolved"]+">>();")
            #else:
            #    return_val = "ret"
            #    if return_group["is_array"]:
            #        code.append("};")
            #
    return "\n".join(code)

def java_call_signature_format(types, return_type, is_constructor=False):
    results = []
    for ty in types:
        results.append(java_letter_from_rust(ty["type_name_original"]))
    if is_constructor:
        return "("+"".join(results)+")V"
    else:
        return "("+"".join(results)+")"+java_letter_from_rust(return_type)

def correct_question_mark(argname, ty, method, i, returning, library, is_constructor):
    gen = re.search("^(.*?)<(.*?)>$", ty)
    if gen is not None:
        name = gen.group(2)
        if "," in name:
            names = name.split(",")
        else:
            names = [name]
        final = []
        for name in names:
            if "?" in name:
                name = name.replace("? extends","").replace("? super","").replace("<?>","").replace(" ","")
                # we don't support further generics at this point.
                gen2 = re.search("<(.*?)>", name)
                if gen2 is not None:
                    return None
                # if its still there at this point, move on.
                if "?" in name:
                    return None
            grp = java_type_to_rust(argname, name, method, i, returning, library, is_constructor=False, skip_vec=True)
            if grp is None:
                return None
            final.append(grp)
        return final
    else:
        grp = java_type_to_rust(argname, ty, method, i, returning, library, is_constructor=False, skip_vec=True)
        if grp is None:
            return None
        gen2 = re.search("<(.*?)>", grp["type_name_resolved"])
        if gen2 is not None:
            return None
        return grp

def java_type_to_rust_primitive(ty):
    type_name_resolved = ""
    match ty.replace("Java",""):
        case "void":
            type_name_resolved = "()"
        case "char" | "java.lang.Character":
            type_name_resolved = "u16"
        case "java.lang.Byte" | "byte":
            type_name_resolved = "i8"
        case "java.lang.Short" | "short":
            type_name_resolved = "i16"
        case "java.lang.Boolean" | "boolean":
            type_name_resolved = "bool"
        case "java.lang.Integer" | "int":
            type_name_resolved = "i32"
        case "java.lang.Long" | "long":
            type_name_resolved = "i64"
        case "java.lang.Float" | "float":
            type_name_resolved = "f32"
        case "java.lang.Double" | "double":
            type_name_resolved = "f64"
        case "java.lang.String":
            type_name_resolved = "String"
    if type_name_resolved != "":
        return type_name_resolved
    else:
        return None

def java_type_to_rust(argname, ty, method, i, returning, library, is_constructor=False, skip_vec=False):
    if method is not None and not is_constructor:
        if returning:
            if "genericReturnType" not in method["method"]:
                parameter_type = method["method"]["name"]
            else:
                parameter_type = method["method"]["genericReturnType"]
        else:
            if i <= len(method["method"]["genericParameterTypes"]):
                parameter_type = method["method"]["genericParameterTypes"][i]
            else:
                parameter_type = ""
    else:
        parameter_type = ""

    type_name_resolved = "jni::objects::JObject"
    is_array = False
    is_interface = False
    if ty.endswith("[]"):
        ty = ty.replace("[]", "")
        is_array = True

    if not isinstance(library, bool):
        if library.startswith("java"):
            ty = "Java"+ty

    type_alone = ""
    generics = []

    usage_unsafe = False

    if ty.replace("com.destroystokyo.paper","org.bukkit") in interface_names:
        is_interface = True

    type_name_original = ty.replace("Java","")
    skip_primitives = False
    if "lang" in library:
        if is_constructor:
            skip_primitives = True
        else:
            skip_primitives = False

    if not skip_primitives:
        t = java_type_to_rust_primitive(ty)
        if t is not None:
            type_name_resolved = t
    if type_name_resolved == "jni::objects::JObject":
        match ty:
            case "java.lang.Class":
                type_name_resolved = "jni::objects::JClass"
            case "java.awt.Color":
                type_name_resolved = "(u8, u8, u8)"
            case "java.lang.Object" | "java.lang.reflect.Type" | "java.awt.Image" | "java.awt.image.BufferedImage":
                type_name_resolved = "jni::objects::JObject"
            case "java.util.List" | "java.util.Collection":
                if skip_vec:
                    return None
                type_name_resolved = "Vec"
                grp = correct_question_mark(argname, parameter_type, method, i, returning, library, is_constructor)
                if grp is None:
                    return None
                generics = grp
            #case "java.util.HashMap":
            #    if skip_vec:
            #        return None
            #    type_name_resolved = "std::collections::HashMap"
            #    parts = []
            #    grp = correct_question_mark(argname, parameter_type, method, i, returning, library, is_constructor)
            #    if grp is None:
            #        return None
            #    parts += grp
            #    generics = parts
            case _:
                ty = ty.replace("Javajava","java")
                crate_name = ".".join(
                    filter(lambda f: f.lower() == f, ty.split(".")))

                while crate_name not in library_resolves and crate_name != "":
                    parts = crate_name.split(".")
                    parts.pop()
                    crate_name = ".".join(parts)

                if crate_name == "":
                    usage_unsafe = True
                else:
                    if (library_name_format(crate_name,library) == library_name_format("java.util",library)) or (library_name_format(crate_name,library) == library_name_format("java.lang",library)):
                        class_name = "".join(filter(lambda f: f[0].upper() == f[0], ty.split(".")))
                        what = ty
                        ty = ty.replace(class_name, "Java"+class_name)

                    to_replace =  library_name_format(crate_name,library)

                    type_name_resolved = ty.replace(
                        crate_name, to_replace).replace(".", "::").replace("-", "_").replace("$", "")

                    if "JavaObject" in type_name_resolved:
                        type_name_resolved = "jni::objects::JObject"
    if type_alone == "":
        type_alone = type_name_resolved

    parts = type_name_resolved.split("::")
    for p in parts:
        if p in reserved_words:
            type_name_resolved = type_name_resolved.replace(
                p, "mod_"+p)
    if argname in reserved_words:
        argname = "val_"+argname

    # camel case to snake case
    type_name_lhand = camel_case_to_snake_case(argname).replace("$", "")
    if type_name_lhand in reserved_words:
        type_name_lhand = "arg_"+type_name_lhand

    if method is not None:
        if "options_start_at" in method:
            options_start_at = method["options_start_at"]
        else:
            options_start_at = -1
    else:
        options_start_at = -1
    package_name = ".".join(filter(lambda f: f.islower(), type_name_original.split(".")))

    if(in_excluded_classes(ty)):
        return None

    return {
        "type_name_resolved": type_name_resolved,
        "type_name_lhand": type_name_lhand,
        "is_array": is_array,
        "is_interface": is_interface,
        "package_name": package_name,
        "type_name_original": type_name_original,
        "type_name_alone": type_alone,
        "generics": generics,
        "options_start_at": options_start_at,
        "usage_unsafe": usage_unsafe,
    }

def java_type_from_rust(type):
    match(type["type_name_alone"]):
        case "bool":
            class_name = "boolean"
            function_signature = "(Z)V"
        case "byte":
            class_name = "byte"
            function_signature = "(B)V"
        case "u16":
            class_name = "char"
            function_signature = "(C)V"
        case "i16":
            class_name = "short"
            function_signature = "(S)V"
        case "i32":
            class_name = "int"
            function_signature = "(I)V"
        case "i64":
            class_name = "long"
            function_signature = "(J)V"
        case "f32":
            class_name = "float"
            function_signature = "(F)V"
        case "f64":
            class_name = "double"
            function_signature = "(D)V"
        case _:
            class_name = (type["package_name"].replace(".","/"))+"/"+type["type_name_alone"]
            function_signature = "(Ljava/Lang/Object)V"
    return {
        "class_name": class_name,
        "function_signature": function_signature
    }

def java_letter_from_rust(type):
    match(type.replace("Java","")):
        case "boolean":
            return "Z"
        case "byte":
            return "B"
        case "char":
            return "C"
        case "short":
            return "S"
        case "int":
            return "I"
        case "long":
            return "J"
        case "float":
            return "F"
        case "double":
            return "D"
        case "void":
            return "V"
        case "":
            return ""
        case _:
            return "L"+type.replace(".","/").replace("Java","")+";"

def gen_to_string_func(name, static):
    if static:
        call = "Self::internal_to_string(&self.0)"
    else:
        call = "self.to_string()"
    return """
        impl<'mc> std::string::ToString for """+name+"""<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling """+name+""".toString: {}",
                        err
                    ),
                }
            }
        }
        """

def gen_instance_of_func(mod_path):
    impl_signature = []
    impl_signature.append("""
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    """)
    for impl in impl_signature:
        file_cache[mod_path].append(impl)

def gen_jniraw_impl(name, is_enum, mod_path, full_name, variants):
    impl_signature = []
    impl_signature.append("""
    impl<'mc> JNIRaw<'mc> for """+name+"""<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        """)
    if is_enum:
        impl_signature.append("match self {")
        for (_,val_proper) in variants:
            impl_signature.append("Self::"+val_proper+" { inner } => inner.0.clone(),")
        impl_signature.append("}")
    else:
        impl_signature.append("self.0.clone()")
    impl_signature.append("}")
    impl_signature.append("fn jni_object(&self) -> jni::objects::JObject<'mc> {")
    if is_enum:
        impl_signature.append("match self {")
        for (_,val_proper) in variants:
            impl_signature.append("Self::"+val_proper+" { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },")
        impl_signature.append("}")
    else:
        impl_signature.append("unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
    impl_signature.append("}")
    impl_signature.append("}")

    impl_signature.append(gen_jni_instantiatable(name,full_name,is_enum,variants))

    for impl in impl_signature:
        file_cache[mod_path].append(impl)

    if is_enum:
        gen_jniraw_impl(name+"Struct",False,mod_path,full_name,variants)

def gen_jni_instantiatable(name,full_name,is_enum,variants):
    st = """impl<'mc> JNIInstantiatable<'mc> for """+name+"""<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    \"Tried to instantiate """+name+""" from null object.\")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, \""""+full_name.replace(".","/")+"""\")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a """+name+""" object, got {}",
                    name
                )
                .into())
            } else {
    """
    if is_enum:
        st += """
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    """
        for (v,val_proper) in variants:
            st += "\""+v+"\" => Ok("+name+"::"+val_proper+" { inner: "+name+"Struct::from_raw(env,obj)?}),"
        st += "_ => Err(eyre::eyre!(\"String gaven for variant was invalid\").into())"
        st += "}"
    else:
        st += "Ok(Self(env.clone(), obj))"
    st += """
            }
        }
    }
    """
    return st

def parse_methods(library,name,methods,mod_path,is_enum,is_trait,is_trait_decl,variants,is_constructor,full_name):
    og_name = name
    impl_signature = []
    extern_signature = []
    methods = list(filter(lambda f: f["name"] != "valueOf",methods))

    has_to_string = False
    has_to_string_is_static = False

    names = {}
    methods_ = {}
    for method in methods:

        if "modifiers" in method:
            if(method["modifiers"]&1 != 1): # skip private methods
                continue
            if(method["modifiers"]&1024 == 1): # skip abstract methods
                continue

        if is_constructor:
            mname = "new"
        else:
            mname = method["name"]

        og_mname = mname

        # camel case to snake case
        mname = camel_case_to_snake_case(mname).replace("$", "")

        if len(method["parameters"]) <= 0:
            mname = mname.replace("get_","")

        if mname in reserved_words:
            mname = "get_"+mname


        names[og_mname] = mname
        if mname in methods_:
            methods_[mname].append(method)
        else:
            methods_[mname] = [method]

    new_methods = {}
    for k in names:

        name = names[k]
        method = methods_[name]
        # are there more then one method with this name?
        if len(methods_[name]) > 1:

            # ok time for some surgery then!

            can_use_options = True
            last_method = None

            # sort the arrays by length
            method_first_arg = ""
            method_map = {}
            method_buildup = []
            options_start_at = 0
            methods__ = sorted(methods_[name], key = lambda key : len(key["parameters"]))
            n = 0
            for m in methods__:
                breaking = False
                if "modifiers" in m:
                    if(m["modifiers"]&1 != 1): # skip protected/private methods
                        continue
                if len(m["parameters"]) >= 1:
                    method_first_arg = camel_case_to_snake_case(m["parameters"][0][1])
                if last_method is None:
                    options_start_at = len(m["parameters"])+1
                    m["options_start_at"] = options_start_at
                    last_method = m
                    method_buildup.append(m)
                else:
                    i = 0
                    if len(last_method["parameters"]) > len(m["parameters"]):
                        can_use_options = False
                        break
                    # for each paramater in this function
                    for i in range(0, len(last_method["parameters"])):
                        if last_method["parameters"][i][1] != m["parameters"][i][1]:
                            can_use_options = False
                            break

                    m["options_start_at"] = options_start_at
                    method_buildup.append(m)

                    if not can_use_options:
                        breaking = True
                    else:
                        last_method = m
                    i += 1
                n += 1

                if n == len(methods__) and not breaking:
                    method_buildup.append(m)
                    breaking = True
                
                if breaking:
                    method_buildup.append(m)
                    identifier = method_first_arg.split(".")[len(method_first_arg.split("."))-1].lower()
                    method_map[identifier] = method_buildup.copy()
                    method_buildup = []
                    method_first_arg = ""
                    options_start_at = m["options_start_at"]

            for group_name in method_map:
                methods = method_map[group_name]
                if group_name != "":
                    new_name = name+"_with_"+group_name.replace("$","").replace("[]","s")
                else:
                    new_name = name 
                new_methods[new_name] = {}
                new_methods[new_name]["method"] = methods[len(methods)-1]
                if "@Deprecated" in new_methods[new_name]["method"]["annotations"]:
                    if "@Deprecated" in methods[0]["annotations"]:
                        new_methods[new_name]["method"]["annotations"].remove("@Deprecated")
                new_methods[new_name]["original_name"] = k
        else:
            new_methods[name] = {}
            new_methods[name]["method"] = method[0]
            new_methods[name]["original_name"] = k

    for k in new_methods:
        method = new_methods[k]

        static = method["method"]["modifiers"]&8 or is_constructor
        types = method["method"]["parameters"]

        name = k
        if name == "to_string":
            if len(types) <= 0:
                has_to_string = True
                has_to_string_is_static = static
                name = "internal_to_string"


        if "options_start_at" in method["method"]:
            options_start_at = method["method"]["options_start_at"]
        else:
            options_start_at = -1

        func_signature = []
        code = []


        if not static:
            func_signature.append({
                "type_name_resolved": "&self",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "type_name_original": "",
                "type_name_alone": "",
                "generics": "",
                "package_name": "",
                "options_start_at": options_start_at,
                "usage_unsafe": False,
            })
        else:
            func_signature.append({
                "type_name_resolved": "jni: &blackboxmc_general::SharedJNIEnv<'mc>",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "type_name_original": "",
                "type_name_alone": "",
                "generics": "",
                "package_name": "",
                "options_start_at": options_start_at,
                "usage_unsafe": False,
            })
        should_continue = True
        i = 0
        # make the function signature
        for type in types:
            group = java_type_to_rust(type[0], type[1],method, i, is_constructor, library)

            if group is None:
                should_continue = False
                break

            func_signature.append(group)
            i += 1

        if not should_continue:
            continue

        # make the inner function.

        obj_call = ""

        prefix = "self.jni_ref()"
        obj_call = "self.jni_object()"

        if is_trait_decl:
            prefix = "self.jni_ref()"
            obj_call = "self.jni_object()"
        if static:
            prefix = "jni"

        # lets parse the types into java.
        n = 0
        types = []

        if is_constructor:
            return_group = java_type_to_rust("", method["method"]["name"], method, i, True, library, True)
        else:
            return_group = java_type_to_rust("", method["method"]["returnType"], method, i,True, library, False)

        if return_group is None:
            continue

        if options_start_at != -1:
            if len(func_signature) > 1:
                code.append("let mut args = Vec::new();")
            else:
                code.append("let args = Vec::new();")
            code.append("let mut sig = String::from(\"(\");")
        else:
            code.append("let sig = String::from(\""+java_call_signature_format(func_signature, return_group["type_name_original"],is_constructor)+"\");")
        for type in func_signature:
            if(type["type_name_lhand"] == "") or (type["is_array"]) or in_excluded_classes(type["type_name_original"]):
                n += 1
                continue
            else:
                ty = code_format(type, prefix, n, class_name=og_name, options_start_at=options_start_at, no_sig=False)
                if ty is None:
                    should_continue = False
                    break
                for t in ty:
                    code.append(t)
            types.append("jni::objects::JValueGen::from(val_"+str(n)+")")
            n += 1

        if options_start_at != -1:
            if not is_constructor:
                code.append("sig += \")"+java_letter_from_rust(return_group["type_name_original"])+"\";")
            else:
                code.append("sig += \")V\";")
        if(not should_continue):
            continue



        if("?" in return_group["type_name_resolved"]):
            continue

        if in_excluded_classes(return_group["type_name_original"]):
            continue

        if return_group["type_name_alone"] in omitted_classes:
            continue

        for generic in return_group["generics"]:
            parts = generic["type_name_resolved"].split("::")
            if parts[len(parts)-1].replace("_","$") in omitted_classes:
                should_continue = False
                break

        if(not should_continue):
            continue

        nullable = False
        impl_signature_canidate = []
        if "comment" in method["method"]:
            comment_raw = method["method"]["comment"].replace("\n","")
        else:
            comment_raw = ""
        if "annotations" in method["method"]:
            i = parse_annotations(method["method"]["annotations"],comment_raw)
            impl_signature_canidate = i[0]
            nullable = i[1]
            if i[2] is not None:
                method["method"]["comment"] = i[2]

        # execute the function.
        m = return_format(return_group, prefix, static, method, obj_call, func_signature, types, is_trait,options_start_at, is_constructor,nullable,library)
        if m is None:
            continue

        impl_signature += impl_signature_canidate
        code.append(m)

        func_signature_resolved = ""
        func_signature_resolved_parts = []
        func_signature_resolved_parts_names_only = []
        generic_letters = []
        generic_args = []
        j = 0
        usage_unsafe = False

        for ty in func_signature:
            if ty["usage_unsafe"]:
                usage_unsafe = True
            group = func_signature_format(ty,j,False,options_start_at)

            if group["result"] != "":
                func_signature_resolved_parts.append(group["result"])
                func_signature_resolved_parts_names_only.append(group["result_name_only"])
                j += 1

            generic_letters += group["generic_letters"]
            generic_args += group["generic_args"]




        return_type = func_signature_format(return_group,j,True)

        generic_letters_str = ""
        if len(generic_letters) >= 1:
            generic_letters_str = "<"+",".join(generic_letters)+">"
        generic_args_str = ""
        if len(generic_args) >= 1:
            generic_args_str = " where "+",".join(generic_args)

        func_signature_resolved = ",".join(func_signature_resolved_parts)
        func_signature_resolved_raw = ",".join(list(map(lambda f: f.replace("&self","this: "+og_name).replace(": ",": *mut ").replace("<'mc>","<'static>"),func_signature_resolved_parts)))
        
        func_signature_resolved_names_only = ",".join(list(map(lambda f: "*"+f+".as_ref().unwrap()", filter(lambda f: f != "&self",func_signature_resolved_parts_names_only))))

        if "comment" in method["method"]:
            impl_signature.append(format_comment(method["method"]["comment"]))
        if name.startswith("internal_"):
            impl_signature.append("#[doc(hidden)]")
        impl_signature.append(
            "\tpub fn "+name+generic_letters_str+"("+func_signature_resolved+") "
        )

        if nullable:
            return_type["result"] = "Option<"+return_type["result"]+">"

        impl_signature.append("-> Result<"+return_type["result"]+", Box<dyn std::error::Error>>")

        impl_signature.append(generic_args_str)

        if is_trait and not is_trait_decl:
            impl_signature.append(";")
        else:
            impl_signature.append("{"+"\n".join(code)+"}")

        raw_return = "*mut "+return_type["result"].replace("<'mc>","<'static>")
        # c binding.
        #if static:
        #    extern_signature.append("#[no_mangle]\npub extern fn "+camel_case_to_snake_case(og_name)+"_"+name+"("+func_signature_resolved_raw+") -> Result<"+raw_return+", Box<dyn std::error::Error>> ")
        #    extern_signature.append("{\n\tunsafe {\n\t\tOk(Box::leak(Box::new("+og_name+"::"+name+"("+func_signature_resolved_names_only+")?)) as "+raw_return+")\n\t}\n}")
        #else:
        #    extern_signature.append("#[no_mangle]\npub extern fn "+camel_case_to_snake_case(og_name)+"_"+name+"("+func_signature_resolved_raw+") -> Result<"+raw_return+", Box<dyn std::error::Error>> ")
        #    extern_signature.append("{\n\tunsafe {\n\t\tOk(Box::leak(Box::new(this.as_ref().unwrap()."+name+"("+func_signature_resolved_names_only+")?)) as "+raw_return+")\n\t}\n}")

    for impl in impl_signature:
        file_cache[mod_path].append(impl)


    return {
        "has_to_string": has_to_string,
        "has_to_string_is_static": has_to_string_is_static,
        "extern_signature": extern_signature,
    }

def format_comment(comment):
    final_comment = ""
    parts = comment.replace("<br>"," \n \n").split("\n")
    for comm in parts:
        while "  " in comm or "\n" in comm or "\r" in comm:
            comm = comm.replace("  ","").replace("\n","").replace("\r","")
        if comm != "":
            if comm[0] == " ":
                comm = comm[1:]
            final_comment +=  "/// "+comm+"\n"
    if final_comment.endswith("\n"):
        final_comment = final_comment[0:len(final_comment)-1]
    return final_comment

def parse_classes(library, val, classes):
    if "modifiers" in val:
        modifiers = int(val["modifiers"])
        if (modifiers&1 != 1):
            omitted_classes.append(val["name"])
            return

    dir = val["packageName"].replace(".", os.sep)
    mod_path = dir+os.sep+"mod.rs"
    name = val["name"].replace("$", "").replace("-", "_")

    if name == "Result" or name == "Option":
        name = "Spigot"+name
        val["name"] = name

    full_name = val["packageName"]+"."+val["name"]
    if library.startswith("java"):
        name = "Java"+name
    if in_excluded_classes(full_name):
        print("excluding "+full_name)
        return

    if mod_path in parsed_classes:
        if full_name in parsed_classes[mod_path]:
            return
        else:
            parsed_classes[mod_path].append(full_name)
    else:
        parsed_classes[mod_path] = [full_name]

    if mod_path not in file_cache:
        file_cache[mod_path] = ["#![allow(deprecated)]\nuse blackboxmc_general::JNIRaw;\nuse blackboxmc_general::JNIInstantiatable;\nuse blackboxmc_general::JNIInstantiatableEnum;\nuse color_eyre::eyre::Result;"]

    if (name == ""):
        return

    if "comment" in val:
        file_cache[mod_path].append(format_comment(val["comment"]))

    if "values" in val:
        if(len(val["values"]) >= 1):
            val["isEnum"] = True
    if val["isEnum"]:  # enum generation
        if(len(val["values"]) < 1):
            if "fields" in val:
                val["values"] += list(filter(lambda f: not "$" in f, val["fields"]))
        file_cache[mod_path].append(
            "pub enum "+name+"<'mc> {")

        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            if "annotations" in val:
                if v in val["annotations"]:
                    file_cache[mod_path] += parse_annotations(val["annotations"][v],"")[0]
            file_cache[mod_path].append("\t"+val_proper+" {inner: "+name+"Struct<'mc>},")


        file_cache[mod_path].append("}")

        variants = []
        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            variants.append((v,val_proper))

        # DISPLAY IMPL

        file_cache[mod_path].append("impl<'mc> std::fmt::Display for "+name+"<'mc> {")
        file_cache[mod_path].append("   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
        file_cache[mod_path].append("       match self {")
        for (v,val_proper) in variants:
            file_cache[mod_path].append("           "+name+"::"+val_proper+" { .. } => f.write_str(\""+v+"\"),")
        file_cache[mod_path].append("       }")
        file_cache[mod_path].append("   }")
        file_cache[mod_path].append("}")

        file_cache[mod_path].append("""
        impl<'mc> """+name+"""<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<"""+name+"""<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class(\""""+full_name.replace(".","/")+"""\");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)L"""+full_name.replace(".","/")+""";",
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
                    """)
        for (v,val_proper) in variants:
            file_cache[mod_path].append("\""+v+"\" => Ok("+name+"::"+val_proper+" { inner: "+name+"Struct::from_raw(env,obj)?}),")

        file_cache[mod_path].append("""
                    _ => Err(eyre::eyre!(\"String gaven for variant was invalid\").into())
                }
            }
        }
        """)

        file_cache[mod_path].append(
            "#[repr(C)]\npub struct "+name+"Struct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);")

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library, cl, classes)

        gen_jniraw_impl(name, True, mod_path, full_name, variants)

        file_cache[mod_path].append("impl<'mc> "+name+"Struct<'mc> {")

        has_to_string = False
        has_to_string_is_static = False
        extern = []
        if "methods" in val:
            grp = parse_methods(library, name,val["methods"],mod_path,True,False,False,variants,False,full_name)
            has_to_string = grp["has_to_string"]
            has_to_string_is_static = grp["has_to_string_is_static"]
            extern += grp["extern_signature"]

        gen_instance_of_func( mod_path)

        file_cache[mod_path].append("}")

        for ex in extern:
            file_cache[mod_path].append(ex)

        if has_to_string:
            file_cache[mod_path].append(gen_to_string_func(name,has_to_string_is_static))

    elif val["isInterface"]: # interface generation
        file_cache[mod_path].append(
            "///\n/// This is a representation of an abstract class."
        )
        file_cache[mod_path].append(
            "#[repr(C)]\npub struct "+name+"<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);"
        )

        gen_jniraw_impl(name, False, mod_path, full_name, None)

        file_cache[mod_path].append(
            "impl<'mc> "+name+"<'mc> {")

        has_to_string = False
        has_to_string_is_static = False

        extern = []
        if "methods" in val:
            grp = parse_methods(library,name,val["methods"],mod_path,False,True,True,[],False,full_name)
            has_to_string = grp["has_to_string"]
            has_to_string_is_static = grp["has_to_string_is_static"]
            extern += grp["extern_signature"]


        gen_instance_of_func( mod_path)

        file_cache[mod_path].append("}")

        for ex in extern:
            file_cache[mod_path].append(ex)

        if has_to_string:
            file_cache[mod_path].append(gen_to_string_func(name,has_to_string_is_static))

        #if "interfaces" in val:
        #    for inter in val["interfaces"]:
        #        print(inter["name"])
    else:  # struct generation
        file_cache[mod_path].append(
            "#[repr(C)]\npub struct "+name+"<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);"
        )

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library,cl, classes)

        gen_jniraw_impl(name, False, mod_path, full_name, None)

        file_cache[mod_path].append("impl<'mc> "+name+"<'mc> {")

        if "constructors" in val:
            grp = parse_methods(library, name,val["constructors"],mod_path,False,False,False,[],True,full_name)

        has_to_string = False
        has_to_string_is_static = False

        extern = []
        if "methods" in val:
            grp = parse_methods(library,name,val["methods"],mod_path,False,False,False,[],False,full_name)
            has_to_string = grp["has_to_string"]
            has_to_string_is_static = grp["has_to_string_is_static"]
            extern += grp["extern_signature"]

        gen_instance_of_func( mod_path)

        file_cache[mod_path].append("}")
        for ex in extern:
            file_cache[mod_path].append(ex)

        if has_to_string:
            file_cache[mod_path].append(gen_to_string_func(name,has_to_string_is_static))

    if not val["isEnum"]:
        if "interfaces" in val:
            for inter in val["interfaces"]:
                parse_into_impl(inter,name,mod_path)
        if "superClass" in val:
            super_class = val["superClass"]
            parse_into_impl(super_class,name,mod_path)
    
    
    # make a blank, "Class" struct that can be passed as a function.
    #file_cache[mod_path].append("""
    #    #[repr(C)]
    #    pub struct """+name+"""Class;
    #    impl blackboxmc_general::JNIProvidesClassName for """+name+"""Class {
    #        fn class_name(&self) -> &str {
    #            \""""+val["packageName"].replace(".","/")+"/"+val["name"]+"""\"
    #        }
    #    }
    #""")


def parse_into_impl(val,name,mod_path):
    if val["packageName"].startswith("java"):
        if not val["packageName"].startswith("java.util"):
            return
    val_resolved = java_type_to_rust("", val["packageName"]+"."+val["name"], None, 0, True, library, False)

    if val_resolved is None:
        return
    if in_excluded_classes(val_resolved["type_name_original"]):
        return
    if val_resolved["type_name_alone"] in omitted_classes:
        return

    # we want to check if the generics are the same.
    if ".".join(val_resolved["package_name"].split(".")[0:2]) in libraries:
        temp_lib = libraries[".".join(val_resolved["package_name"].split(".")[0:2])]
        temp_pkg = temp_lib[val_resolved["package_name"]]
        typealone = "".join(filter(lambda f: f[0].isupper(), val_resolved["type_name_original"].split(".")))
        if typealone in temp_pkg:
            temp_cls = temp_pkg[typealone]

    if "impl<'mc> Into<"+val_resolved["type_name_resolved"]+"<'mc>> for "+name+"<'mc>{\n" in file_cache[mod_path]:
        return

    file_cache[mod_path].append("impl<'mc> Into<"+val_resolved["type_name_resolved"]+"<'mc>> for "+name+"<'mc>{\n")
    file_cache[mod_path].append("fn into(self) -> "+val_resolved["type_name_resolved"]+"<'mc> {\n")
    if val_resolved["type_name_resolved"] == "jni::objects::JObject":
        file_cache[mod_path].append("self.1")
    else:
        file_cache[mod_path].append(val_resolved["type_name_resolved"]+"::from_raw(&self.jni_ref(), self.1).expect(\"Error converting "+name+" into "+val_resolved["type_name_resolved"]+"\")\n")
    file_cache[mod_path].append("   }\n"+
                            "}")

dep_comment_regex = "<div class=\"deprecation-comment\">(.*?)<\/div>"

def add_deprecated(arr,comment):
    if "deprecation-comment" in comment:
        html_parsed_with_regex_lmao = re.search(dep_comment_regex,comment)
        if html_parsed_with_regex_lmao is not None:
            comm = html_parsed_with_regex_lmao.group(1)
            while "  " in comm:
                comm = comm.replace("  "," ")
            if comm.startswith(" "):
                comm = comm[1:]
            if comm.endswith(" "):
                comm = comm[:len(comm)]
            comment = comment.replace(comm,"") # idk why the code below doesn't catch this but ok
            arr.append("#[deprecated = \""+comm.replace("\"","'")+"\"]")
        else:
            arr.append("#[deprecated]")
    else:
        arr.append("#[deprecated]")
    comment = re.sub(dep_comment_regex,"",comment)
    comment = re.sub("<span class=\"deprecated-label\">(.*?)<\/span>","",comment)
    return comment

def parse_annotations(annotations,comment):
    strings = []
    nullable = False
    new_comment = None
    for annotation in annotations:
        if(len(annotation) >= 3):
            match(annotation):
                case "@Deprecated":
                    new_comment = add_deprecated(strings,comment)
                case "@Experimental":
                    nop = 0
                case "@Nullable":
                    nullable = True
        elif(len(annotation) == 2):
            match(annotation[0]):
                case "forRemoval":
                    new_comment = add_deprecated(strings,comment)
    return (strings,nullable,new_comment)
# what we first want to do is collect any interfaces.
for library in libraries:
    packages = libraries[library]
    for package in packages:
        classes = packages[package]
        for clas in classes:
            if "isInterface" in classes[clas]:
                if classes[clas]["isInterface"]:
                    interface_names.append(classes[clas]["packageName"]+"."+classes[clas]["name"])
            if "isEnum" in classes[clas]:
                if classes[clas]["isEnum"]:
                    enums.append(classes[clas]["packageName"]+"."+classes[clas]["name"])
            if "classes" in classes[clas]:
                for cla in classes[clas]["classes"]:
                    if "isInterface" in cla:
                        if cla["isInterface"]:
                            interface_names.append(cla["packageName"]+"."+cla["name"])
                    if "isEnum" in cla:
                        if cla["isEnum"]:
                            enums.append(cla["packageName"]+"."+cla["name"])


for library in libraries:
    packages = libraries[library]

    crate_dir = ""

    if library in library_resolves:
        res = library_resolves[library]
        if "src" in res:
            crate_dir = res + os.sep
        else:
            crate_dir = os.path.join(res, "src")
    else:
        print("Unhandled library "+library +
            ". All libraries must have corresponding rust crates.")
        exit(0)

    root = library_name_no_extra_paths(crate_dir)
    if "src" not in root:
        root += os.sep + "src"

    # delete and recreate the appropriate directory if it's the first time writing to it
    if pathlib.Path(crate_dir).exists():
        shutil.rmtree(crate_dir)
        path = library.replace(".", os.sep)
        pathlib.Path(crate_dir+os.sep +
                    path).mkdir(parents=True, exist_ok=True)


    # make the appropriate directory if it doesn't already exist.
    path = library.replace(".", os.sep)
    pathlib.Path(crate_dir+os.sep +
                path).mkdir(parents=True, exist_ok=True)
    print(path)

    for package in packages:
        classes = packages[package]
        added = []
        for clas in classes:
            parse_classes(library, classes[clas], classes)

    for k in file_cache:
        val = file_cache[k]
        ok = k.split(os.sep)
        ok.pop()
        ok = os.sep.join(ok)
        pathlib.Path(crate_dir+os.sep +
                    ok).mkdir(parents=True, exist_ok=True)
        with open(crate_dir+os.sep+k, "w") as file:
            file.truncate(0)
            for v in val:
                file.write(v+"\n")
            file.close()

    file_cache = {}

    # move everything in org/bukkit into root
    parts = library.split(".")
    dir = crate_dir + os.sep + os.sep.join([parts[0], parts[1]])
    shutil.copytree(dir, crate_dir, dirs_exist_ok=True)
    shutil.rmtree(crate_dir + os.sep + parts[0])

    mod_rs = crate_dir+os.sep+"lib.rs"

    if mod_rs not in file_cache:
        file_cache[mod_rs] = []

    with open(mod_rs, "a+") as file:
        for v in file_cache[mod_rs]:
            file.write(v+"\n")
        file.close()

    file_cache = {}


    if root not in filled_once:
        mod_rs_folder_populate(root)
        os.rename(root+os.sep+"mod.rs", root+os.sep+"lib.rs")
    filled_once.append(root)

# inject any manually written code.
for filename in os.listdir("additions"):
    with open(os.path.join("additions", filename), "r") as f:
        lines = f.readlines()
        opening_line = lines[0]
        if(not opening_line.startswith("//")):
            continue
        # if it has "#", then that's a struct that this is appended to.
        if("#" in opening_line):
            parts = opening_line.split("#")
            opening_line = parts[0]
            append_to_struct = parts[1].replace("\n","").replace("\r","")
        else:
            append_to_struct = None
        filename2 = "./"+re.sub("[^A-Za-z/\-\.]", "", opening_line.replace("//","")).replace("/",os.sep)
        parent_dir = filename2.replace(os.sep+"mod.rs", "");
        if not os.path.exists(parent_dir):
            os.makedirs(parent_dir)
        with open(filename2, "a+") as f2:
            lines = "".join(lines[1:])
            if append_to_struct is not None:
                f2.seek(0)
                newlines = f2.readlines()
                at = int(newlines.index("impl<'mc> "+append_to_struct+"<'mc> {\n"))
                newlines.insert(at+1, lines)
                f2.truncate(0)
                f2.write("".join(newlines))
            else:
                f2.write(lines)
            f2.close()
        f.close()

for i in range(0,4):
    wildcard = "*/*" * i
    for library in library_resolves:
        resolved = library_resolves[library]
        if "src" not in resolved:
            resolved += "/src"
        os.system("rustfmt --unstable-features --skip-children ./"+resolved+"/*"+wildcard+".rs")

os.system("cargo fix --allow-dirty --allow-staged --broken-code --jobs "+str(multiprocessing.cpu_count()))