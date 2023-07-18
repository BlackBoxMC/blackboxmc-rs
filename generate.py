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
    "net.md_5": "bungee",
    "org.bukkit": "bukkit",
}

parsed_classes = {}
excluded_classes = [
    "org.bukkit.plugin.SimplePluginManager",    # uses stuff that isn't being generated due to that java bug and i don't want to write an entire class binding for something that's getting deprecated anyways.
]

interface_names = []
filled_once = []
bindings = {}
omitted_classes = []
enums = []

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
    thing = ""
    internal = ty["type_name_resolved"].startswith("crate") or ty["type_name_resolved"].startswith("bukkit") or ty["type_name_resolved"].startswith("jni") or ty["type_name_resolved"].startswith("blackbox")
    generic_letters = []
    generic_args = []
    if(ty["type_name_lhand"] == "" and not returning):
        thing += ty["type_name_resolved"]
    else:
        if not returning:
            thing += ty["type_name_lhand"]+": "

        if(options_start_at > -1):
            if increment >= options_start_at:
                thing += "std::option::Option<"

        if(ty["type_name_resolved"].startswith("&dyn")):
            old = ty["type_name_resolved"]
            letter = chr(ord('a')+increment).upper()
            ty["type_name_resolved"] = letter
            generic_letters.append(letter)
            generic_args.append(letter+": crate::JNIRaw<'mc> + "+old.replace("&dyn","")+"<'mc>")

        if ty["is_array"]:
            thing += "Vec<"+ty["type_name_resolved"]
            if(internal):
                thing += "<'mc>"
            thing += ">"
        else:
            if len(ty["generics"]) >= 1:
                thing += ty["type_name_alone"]+"<"
                j = []
                for g in ty["generics"]:
                    if len(g) >= 2:
                        if g.startswith("crate") or g.startswith("bukkit") or g.startswith("jni") or g.startswith("blackbox"):
                            if not g.endswith("<'mc>"):
                                j.append(g+"<'mc>")
                            else:
                                j.append(g)
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
        if(options_start_at != -1):
            if increment >= options_start_at:
                thing += ">"
    return {
        "result": thing,
        "generic_letters": generic_letters,
        "generic_args": generic_args,
    }

def argument_name_format(objects):
    parts = objects[1].split(".")
    name = parts[len(parts) - 1].replace("[]", "s").replace("$", "")
    return re.sub(r"([a-z])([A-Z])", r"\1_\2", name).lower()

def code_format(type, prefix, n, var_prefix="val", arg="", class_name="", options_start_at=-1):
    if arg == "":
        if "type_name_lhand" in type:
            arg = type["type_name_lhand"]
        else:
            arg = "arg"+str(n)

    if type["is_array"]:
        return None

    if options_start_at != -1:
        if n >= options_start_at-1:
            arg += ".unwrap()"

    ty = type["type_name_resolved"].replace("&dyn ","")

    if(ty.startswith("crate") or ty.startswith("blackbox")): # for internal types...
        # get the original object.
        fcall = ".1.clone()"
        if type["type_name_resolved"].startswith("&dyn"):
            fcall = ".jni_object().clone()"

        return ["let "+var_prefix+"_"+str(n)+" = unsafe { jni::objects::JObject::from_raw("+arg+fcall+")};"]
    else:
        match(type["type_name_alone"]):
            case "bool" | "i8" | "char" | "f64" | "f32" | "i32" | "i64" | "i16" | "u16":
                v = java_type_from_rust(type["type_name_alone"])
                class_name = v["class_name"]
                function_signature = v["function_signature"]
                if class_name != "":
                    match(type["type_name_alone"]):
                        case "bool":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Bool("+arg+".into());"]
                        case "i8":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Byte("+arg+".into());"]
                        case "char" | "u16":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Char("+arg+".into());"]
                        case "f64":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Double("+arg+".into());"]
                        case "f32":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Float("+arg+".into());"]
                        case "i32":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Int("+arg+".into());"]
                        case "i64":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Long("+arg+".into());"]
                        case "i16":
                            return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Short("+arg+".into());"]
                else:
                    return None
            case "u128":
                return [
                    "let upper = ("+arg+" >> 64) as u64 as i64;",
	                "let lower = "+arg+" as u64 as i64;",
	                "let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+prefix+".new_object(\"java/util/UUID\", \"(JJ)V\", &[upper.into(), lower.into()]).unwrap());"
                ]
            case "String":
                return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JObject::from("+prefix+".new_string("+arg+").unwrap());"]
            case "jni::objects::JObject" | "jni::objects::JClass":
                # nothing to do
                return ["let "+var_prefix+"_"+str(n)+" = "+arg+";"]
            case "std::collections::HashMap":
                match type["type_name_original"]:
                    case "java.util.Map":
                        c = [
                            "let raw_"+var_prefix+"_"+str(n)+" = "+prefix+".new_object(\"java/util/HashMap\", \"()V\", &[]).unwrap();",
                            "for (k, v) in "+arg+"{"
                        ]

                        t1 = java_type_from_rust(type["generics"][0])["class_name"]
                        t2 = java_type_from_rust(type["generics"][1])["class_name"]

                        c.append("\n\t\t".join(code_format({
                            "type_name_resolved": type["generics"][0],
                            "type_name_lhand": "k",
                            "is_array": type["is_array"],
                            "is_interface": type["is_interface"],
                            "type_name_original": t1,
                            "type_name_alone": type["generics"][0],
                            "generics": type["generics"],
                            "package_name": type["package_name"],
                            "options_start_at": options_start_at,
                        }, prefix, 0, "map_val", "k", options_start_at)))

                        c.append("\n\t\t".join(code_format({
                            "type_name_resolved": type["generics"][1],
                            "type_name_lhand": "v",
                            "is_array": type["is_array"],
                            "is_interface": type["is_interface"],
                            "type_name_original": t2,
                            "type_name_alone": type["generics"][1],
                            "generics": type["generics"],
                            "package_name": type["package_name"],
                            "options_start_at": options_start_at,
                        }, prefix, 1, "map_val", "v", options_start_at)))

                        c.append(
                            prefix+".call_method(&raw_"+
                                var_prefix+"_"+str(n)+","+
                                "\"put\","
                                "\"(L"+t1+"L"+t2+")V\","+
                                "&[jni::objects::JValueGen::from(&map_val_0), jni::objects::JValueGen::from(&map_val_1)]"
                            ")?;"
                        )

                        c.append("};")

                        c.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(raw_"+var_prefix+"_"+str(n)+");")
                        return c

                    case _:
                        print("Unhandled map type:\t\t"+type["type_name_original"],"\t\t",type["type_name_original"])
                return None
            case "Vec":
                match type["type_name_original"]:
                    case "java.util.List":
                        c = [
                            "let raw_"+var_prefix+"_"+str(n)+" = "+prefix+".new_object(\"java/util/ArrayList\", \"()V\", &[]).unwrap();",
                            "for v in "+arg+"{"
                        ]

                        t1 = java_type_from_rust(type["generics"][0])["class_name"]

                        c.append("\n\t\t".join(code_format({
                            "type_name_resolved": type["generics"][0],
                            "type_name_lhand": "v",
                            "is_array": type["is_array"],
                            "is_interface": type["is_interface"],
                            "type_name_original": t1,
                            "type_name_alone": type["generics"][0],
                            "generics": type["generics"],
                            "package_name": type["package_name"],
                            "options_start_at": options_start_at,
                        }, prefix, 0, "map_val", "v", options_start_at)))

                        c.append(
                            prefix+".call_method("+
                                "&raw_"+var_prefix+"_"+str(n)+","+
                                "\"add\","
                                "\"(L"+t1+")V\","+
                                "&[jni::objects::JValueGen::from(&map_val_0)]"
                            ")?;"
                        )

                        c.append("};")

                        c.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(raw_"+var_prefix+"_"+str(n)+");")

                        return c

                    case _:
                        print("Unhandled map type:\t\t"+type["type_name_original"],"\t\t",type["type_name_original"])
                        return None
            case _:
                print("Untranslated argument:\t\t"+type["type_name_alone"],"\t\t",type["type_name_original"])
                return None

def return_format(return_group, prefix, static, method, obj_call, func_signature, types, is_trait, options_start_at):
    if return_group["is_array"]:
        return None
    else:
        end_line = "?"
    code = []
    if static:
        code.append("let cls = &jni.find_class(\""+return_group["type_name_original"].replace(".","/")+"\")"+end_line+"; let res ="+
            prefix+".call_static_method(cls,"+
                    "\""+method["original_name"]+"\",\""+
                    java_call_signature_format(func_signature, return_group["type_name_original"])+"\",&["+
                    ",".join(types)+
                    "])"+end_line+";")
    else:
        if return_group["type_name_resolved"] != "()":
            code.append("let res =")
        code.append(
            prefix+".call_method("+
                    "&"+obj_call+","+
                    "\""+method["original_name"]+"\",\""+
                    java_call_signature_format(func_signature, return_group["type_name_original"])+"\",&["+
                    ",".join(types)+
                    "])"+end_line+";")

    match return_group["type_name_resolved"]:
        case "()": code.append("Ok(())")
        case "u16": code.append("Ok(res.c().unwrap())")
        case "i8": code.append("Ok(res.b().unwrap())")
        case "i16": code.append("Ok(res.s().unwrap())")
        case "bool": code.append("Ok(res.z().unwrap())")
        case "i32": code.append("Ok(res.i().unwrap())")
        case "i64": code.append("Ok(res.j().unwrap())")
        case "f32": code.append("Ok(res.f().unwrap())")
        case "f64": code.append("Ok(res.d().unwrap())")
        case "u128" | "std::net::IpAddr" | "std::fs::File" | "std::net::SocketAddr":
            return None # not yet
        case "String": code.append(
                    "Ok("+prefix+
                        ".get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })"+end_line+
                        ".to_string_lossy()"+
                        ".to_string())"
            )
        case "jni::objects::JObject": code.append("Ok(res.l().unwrap())")
        case "jni::objects::JClass":
            code.append("Ok(unsafe {"+
                        "jni::objects::JClass::from_raw(res.as_jni().l)"+
                        "})")
        case "(u8, u8, u8)":
            code.append("let r = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getRed\", \"(V)I\", &[])"+
                        "            ?.i()"+end_line+" as u8;"+
                        "let g = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getGreen\", \"(V)I\", &[])"+
                        "            ?.i()"+end_line+" as u8;"+
                        "let b = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getBlue\", \"(V)I\", &[])"+
                        "            ?.i()"+end_line+" as u8;"+
                        "(r, g, b);"+
                        "Ok((r, g, b))")
        case _:
            if len(return_group["generics"]) >= 1:
                nop = 0
                print("Ignoring function",method["method"]["name"],"that returns generic",return_group["type_name_resolved"])
                return None
            else:
                if return_group["type_name_resolved"].startswith("&dyn "):
                    what = return_group["type_name_alone"].split("::")
                    name = what[len(what)-1]
                    what.pop()
                    return_group["type_name_resolved"] = "::".join(what)+name
                if return_group["is_array"]:
                    code.append("let arr = &Into::<jni::objects::JObjectArray>::into(unsafe {"+
                                "    jni::objects::JObject::from_raw(res.as_jni().l)"+
                                "});"+
                                "let num = "+prefix+".get_array_length(arr)"+end_line+";"+
                                "let mut vec = (0..num)"+
                                "    .map(|i| {")
                else:
                    if return_group["type_name_resolved"] != "()":
                        code.append("let ret = {")



                if static:
                    code.append("let mut obj = res.l()"+end_line+";")
                    val_1 = "jni"
                    val_2 = "obj"
                else:
                    if is_trait:
                        val_1 = "self.jni_ref()"
                    else:
                        val_1 = "self.0"
                    val_2 = "unsafe { jni::objects::JObject::from_raw(res.l()"+end_line+".clone()) }"
                if return_group["type_name_original"] in enums:
                    code.append("let raw_obj = "+val_2+";let variant = "+val_1+".call_method(&raw_obj, \"toString\", \"()Ljava/lang/String;\", &[])"+end_line+";"+
                                "let variant_str = "+val_1+""+
                                "    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })"+end_line+
                                "    .to_string_lossy()"+
                                "    .to_string();")
                    val_2 = "raw_obj"
                code.append(return_group["type_name_resolved"]+"("+
                                                prefix+","+
                                                val_2)

                if return_group["type_name_original"] in enums:
                    code.append(", "+return_group["type_name_resolved"]+"::from_string(variant_str).unwrap()")
                code.append(")")
                if return_group["is_array"]:
                    return_val = "Box::leak(Box::new(vec))"
                    code.append("})"+
                                ".collect::<Vec<"+return_group["type_name_resolved"]+">>();")
                else:
                    return_val = "ret"
                    code.append("};")

                if static:
                    code.append("Ok("+return_val+")")
                else:
                    #code.append("let len = self.2."+str(index)+".len();")
                    #code.append("&self.2."+str(index)+".push("+return_val+");")
                    code.append("Ok("+return_val+")")
    return "\n".join(code)

def java_call_signature_format(types, return_type):
    results = []
    for ty in types:
        results.append(java_letter_from_rust(ty["type_name_original"]))
    return "("+"".join(results)+")"+java_letter_from_rust(return_type)

def java_type_to_rust(argname, ty, method, i, returning, library):
    if method is not None:
        if returning:
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

    type_alone = ""
    generics = []

    if ty.replace("com.destroystokyo.paper","org.bukkit") in interface_names:
        is_interface = True


    type_name_original = ty
    match ty:
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
        case "java.util.UUID":
            type_name_resolved = "u128"
        case "java.lang.Class":
            type_name_resolved = "jni::objects::JClass"
        case "java.io.File":
            type_name_resolved = "std::fs::File"
        case "java.io.Reader":
            type_name_resolved = "&dyn std::io::Read"
        case "java.io.Writer":
            type_name_resolved = "&dyn std::io::Write"
        case "java.awt.Color":
            type_name_resolved = "(u8, u8, u8)"
        case "java.net.InetAddress":
            type_name_resolved = "std::net::IpAddr"
        case "java.net.InetSocketAddress":
            type_name_resolved = "std::net::SocketAddr"
        case "java.util.Map" | "java.util.Set":
            final_type = ""
            if(ty == "java.util.Map"):
                final_type = "std::collections::HashMap"
            else:
                final_type = "std::collections::HashSet"
            # get the inner type.
            inner_type_name_parts = re.findall("<(.*?)>", parameter_type)
            if len(inner_type_name_parts) <= 0:
                return None
            inner_type_name_raw = inner_type_name_parts[0]
            inner_types = []
            if "?" in inner_type_name_raw:
                return None

            for ty in inner_type_name_raw.split(","):
                resolved = java_type_to_rust("unk",ty.replace(" ",""),method,i,returning,library)
                if resolved is None:
                    return None
                inner_types.append(resolved["type_name_resolved"])
            type_alone = final_type
            generics = inner_types
            type_name_resolved = final_type+"<" + ",".join(inner_types) + ">"
        case "java.util.List" | "java.util.ArrayList" | "java.util.Collection" | "java.lang.Iterable" | "java.util.Iterator":
            final_type = ""
            prefix = ""
            match ty:
                case "java.util.List" | "java.util.ArrayList" | "java.util.Collection":
                    final_type = "Vec"
                case "java.lang.Iterable" | "java.util.Iterator":
                    final_type = "&dyn std::iter::Iterator"
                    prefix = "Item = "
            # get the inner type.
            inner_type_name_parts = re.findall("<(.*?)>", parameter_type)
            if len(inner_type_name_parts) <= 0:
                return None
            inner_type_name_raw = inner_type_name_parts[0]
            inner_type = java_type_to_rust("unk",inner_type_name_raw.replace(" ",""),method,i,returning,library)
            if inner_type is None:
                return None
            inner_type_name = inner_type["type_name_resolved"]
            if "crate::" in inner_type_name:
                inner_type_name += "<'mc>"
            type_alone = final_type
            if "<T" in inner_type_name:
                # i don't wanna deal with this shit right now maybe later
                return None
            generics = [prefix+inner_type_name]
            type_name_resolved = final_type+"<"+prefix+inner_type_name+">"
        case "java.lang.Object" | "java.util.Random" | "java.lang.reflect.Type" | "java.awt.Image" | "java.awt.image.BufferedImage" | "java.util.regex.Pattern":
            type_name_resolved = "jni::objects::JObject"
        case "java.util.function.Consumer":
            # get the inner type.
            inner_type_name_parts = re.findall("<(.*?)>", parameter_type)
            if len(inner_type_name_parts) <= 0:
                return None
            inner_type_name_raw = inner_type_name_parts[0]
            inner_type = java_type_to_rust("unk",inner_type_name_raw.replace(" ",""),method,i,returning,library)
            if inner_type is None:
                return None
            inner_type_name = inner_type["type_name_resolved"]
            type_alone = "std::ops::Fn"
            generics = [inner_type_name]
            type_name_resolved = "std::boxed::Box<&dyn std::ops::Fn(crate::SharedJNIEnv<'mc>,"+inner_type_name+")>"

        case _:
            if (ty.startswith("java") != True):
                crate_name = ".".join(
                    filter(lambda f: f.lower() == f, ty.split(".")))

                cont = True
                while crate_name not in library_resolves and crate_name != "":
                    parts = crate_name.split(".")
                    parts.pop()
                    crate_name = ".".join(parts)

                if crate_name == "":
                    if len(ty) >= 2: # ignore ?, T, etc... for now
                        # it just so happens that the current external functions are stuff we don't care to implement anyways. but we'll leave this here for the future.
                        #print("Unbound external class:",ty,"\t\t",method["method"]["name"])
                        _ = 0
                    return None
                else:
                    if crate_name in library_resolves:
                        to_replace = "crate::"+library_resolves[crate_name]
                    else:
                        to_replace = library_resolves[crate_name]

                    type_name_resolved = ty.replace(
                        crate_name, to_replace).replace(".", "::").replace("-", "_").replace("$", "")
            else:
                print("Unbound std class:",ty,"\t\t",method["method"]["name"])

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

    if "options_start_at" in method:
        options_start_at = method["options_start_at"]
    else:
        options_start_at = -1

    package_name = ".".join(filter(lambda f: f.islower(), type_name_original.split(".")))
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
    }

def java_type_from_rust(type):
    match(type):
        case "bool":
            class_name = "java/lang/Boolean"
            function_signature = "(Z)V"
        case "byte":
            class_name = "java/lang/Byte"
            function_signature = "(B)V"
        case "u16":
            class_name = "java/lang/Character"
            function_signature = "(C)V"
        case "i16":
            class_name = "java/lang/Short"
            function_signature = "(S)V"
        case "i32":
            class_name = "java/lang/Integer"
            function_signature = "(I)V"
        case "i64":
            class_name = "java/lang/Long"
            function_signature = "(J)V"
        case "f32":
            class_name = "java/lang/Float"
            function_signature = "(F)V"
        case "f64":
            class_name = "java/lang/Double"
            function_signature = "(D)V"
        case _:
            class_name = "java/Lang/Object"
            function_signature = "(Ljava/Lang/Object)V"
    return {
        "class_name": class_name,
        "function_signature": function_signature
    }

def java_letter_from_rust(type):
    match(type):
        case "java.lang.Boolean" | "boolean":
            return "Z"
        case "java.lang.Byte" | "byte":
            return "B"
        case "java.lang.Character" | "char":
            return "C"
        case "java.lang.Short" | "short":
            return "S"
        case "java.lang.Integer" | "int":
            return "I"
        case "java.lang.Long" | "long":
            return "J"
        case "java.lang.Float" | "float":
            return "F"
        case "java.lang.Double" | "double":
            return "D"
        case "void":
            return "V"
        case "":
            return ""
        case _:
            return "L"+type.replace(".","/")+";"

def parse_methods(library,name,methods,mod_path,is_enum,is_trait,is_trait_decl,variants,is_constructor):
    og_name = name
    impl_signature = []
    if not is_trait:
        impl_signature.append("impl<'mc> "+name+"<'mc> {")

    if is_enum and not is_trait:
        for (v,val_proper) in variants:
            impl_signature.append("pub const "+val_proper.upper()+": "+name+"Enum = "+name+"Enum::"+val_proper+";")
            
        impl_signature.append("pub fn from_string(str: String) -> std::option::Option<"+name+"Enum> {\nmatch str.as_str() {")
        for (v,val_proper) in variants:
            impl_signature.append("\""+v+"\" => Some("+name+"Enum::"+val_proper+"),")
        impl_signature.append("_ => None}}")


    if not is_enum:
        impl_signature.append("pub fn from_raw(env: &crate::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>) -> Result<Self, Box<dyn std::error::Error>> {")
        impl_signature.append(
                    "if obj.is_null() {\n"+
                    "    return Err(eyre::eyre!(\n"+
                    "        \"Tried to instantiate "+name+" from null object.\")\n"+
                    "    .into());\n"+
                    "}\n"+
                    "let cls = env.jni.borrow().get_object_class(&obj)?;\n"+
                    "let name_raw = env\n"+
                    "    .call_method(cls, \"getName\", \"()Ljava/lang/String;\", &[])?;\n"+
                    "let oh = name_raw.l()?.into();\n"+
                    "let what = env\n"+
                    "    .get_string(&oh)?;\n"+
                    "let name = what.to_string_lossy();\n"+
                    "if !name.ends_with(\""+name+"\") {\n"+
                    "    Err(eyre::eyre!(\n"+
                    "        \"Invalid argument passed. Expected a "+name+" object, got {}\",\n"+
                    "        name\n"+
                    "    )\n"+
                    "    .into())\n"+
                    "} else {\n"+
                    "    Ok(Self(env.clone(), obj))\n"+
                    "}"
            )
        impl_signature.append("}")

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

        # i just want to fast forward past this one bug, todo: deal with this later
        # (the bug is that this isn't picked up when the trait is being generated, only the trait decl)
        if mname == "create_section":
            continue

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
                if last_method is None:
                    options_start_at = len(m["parameters"])
                    m["options_start_at"] = options_start_at
                    last_method = m
                    method_buildup.append(m)
                    if len(m["parameters"]) >= 1:
                        method_first_arg = camel_case_to_snake_case(m["parameters"][0][1])
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

                    if can_use_options:
                        m["options_start_at"] = options_start_at
                        method_buildup.append(m)
                        last_method = m
                    else:
                        breaking = True
                        break
                    i += 1
                n += 1

                if n == len(methods__) and breaking is not True:
                    breaking = True
                if breaking:
                    identifier = method_first_arg.split(".")[len(method_first_arg.split("."))-1].lower()
                    method_map[identifier] = method_buildup
                    method_buildup = []
                    last_method = None
                    method_first_arg = ""
                    options_start_at = 0

            for group_name in method_map:
                methods = method_map[group_name]
                if group_name != "":
                    new_name = name+"_with_"+group_name.replace("$","").replace("[]","s")
                else:
                    new_name = name
                new_methods[new_name] = {}
                new_methods[new_name]["method"] = methods[len(methods)-1]
                new_methods[new_name]["original_name"] = k
        else:
            new_methods[name] = {}
            new_methods[name]["method"] = method[0]
            new_methods[name]["original_name"] = k

    for k in new_methods:
        method = new_methods[k]

        name = k
        types = method["method"]["parameters"]

        if "options_start_at" in method["method"]:
            options_start_at = method["method"]["options_start_at"]
        else:
            options_start_at = -1

        func_signature = []
        code = []

        static = method["method"]["modifiers"]&8 or is_constructor
        if not static:
            func_signature.append({
                "type_name_resolved": "&mut self",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "type_name_original": "",
                "type_name_alone": "",
                "generics": "",
                "package_name": "",
                "options_start_at": options_start_at,
            })
        else:
            func_signature.append({
                "type_name_resolved": "mut jni: crate::SharedJNIEnv<'mc>",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "type_name_original": "",
                "type_name_alone": "",
                "generics": "",
                "package_name": "",
                "options_start_at": options_start_at,
            })
        should_continue = True
        i = 0
        # make the function signature
        for type in types:
            group = java_type_to_rust(type[0], type[1],method, i, False, library)

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
        for type in func_signature:
            if(type["type_name_lhand"] == ""):
                continue
            if(type["is_array"]):
                continue
            else:
                ty = code_format(type, prefix, n, class_name=og_name, options_start_at=options_start_at)
                if ty is None:
                    should_continue = False
                    break
                for t in ty:
                    code.append(t)
            types.append("jni::objects::JValueGen::from(&val_"+str(n)+")")
            n += 1
        if(not should_continue):
            continue

        return_group = java_type_to_rust("", method["method"]["returnType"], method, i,True, library)

        if return_group is None:
            continue

        if("?" in return_group["type_name_resolved"]):
            continue

        if return_group["type_name_alone"] in omitted_classes:
            continue


        for generic in return_group["generics"]:
            parts = generic.split("::")
            if parts[len(parts)-1].replace("_","$") in omitted_classes:
                should_continue = False
                break

        if(not should_continue):
            continue

        # execute the function.
        m = return_format(return_group, prefix, static, method, obj_call, func_signature, types, is_trait,options_start_at)
        if m is None:
            continue
        code.append(m)

        func_signature_resolved = ""
        func_signature_resolved_parts = []
        generic_letters = []
        generic_args = []
        j = 0
        for ty in func_signature:
            group = func_signature_format(ty,j,False,options_start_at)

            if group["result"] != "":
                func_signature_resolved_parts.append(group["result"])
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
        impl_signature.append(
            "\tpub fn "+name+generic_letters_str+"("+func_signature_resolved+") "
        )

        impl_signature.append("-> Result<"+return_type["result"]+", Box<dyn std::error::Error>>")

        impl_signature.append(generic_args_str)

        if is_trait and not is_trait_decl:
            impl_signature.append(";")
        else:
            impl_signature.append("{"+"\n".join(code)+"}")


    if not is_trait:
        impl_signature.append("}")
    for impl in impl_signature:
        file_cache[mod_path].append(impl)


def parse_classes(library, val, classes):
    if "modifiers" in val:
        modifiers = int(val["modifiers"])
        if (modifiers&1 != 1):
            omitted_classes.append(val["name"])
            return

    dir = val["packageName"].replace(".", os.sep)
    mod_path = dir+os.sep+"mod.rs"
    name = val["name"].replace("$", "").replace("-", "_")

    full_name = val["packageName"]+"."+name

    if full_name in excluded_classes:
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
        file_cache[mod_path] = ["use crate::JNIRaw;"]

    if (name == ""):
        return

    if val["isEnum"]:  # enum generation
        file_cache[mod_path].append(
            "pub enum "+name+"Enum {")

        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            file_cache[mod_path].append("\t"+val_proper+",")


        file_cache[mod_path].append("}")

        variants = []
        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            variants.append((v,val_proper))

        # DISPLAY IMPL

        file_cache[mod_path].append("impl std::fmt::Display for "+name+"Enum {")
        file_cache[mod_path].append("   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
        file_cache[mod_path].append("       match &self {")
        for (v,val_proper) in variants:
            file_cache[mod_path].append("           "+name+"Enum::"+val_proper+" => f.write_str(\""+v+"\"),")
        file_cache[mod_path].append("       }")
        file_cache[mod_path].append("   }")
        file_cache[mod_path].append("}")

        file_cache[mod_path].append(
            "pub struct "+name+"<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub "+name+"Enum);")

        file_cache[mod_path].append("impl<'mc> std::ops::Deref for "+name+"<'mc> {")
        file_cache[mod_path].append("   type Target = "+name+"Enum;")
        file_cache[mod_path].append("   fn deref(&self) -> &Self::Target {")
        file_cache[mod_path].append("       return &self.2;")
        file_cache[mod_path].append("   }")
        file_cache[mod_path].append("}")
        file_cache[mod_path].append("impl<'mc> crate::JNIRaw<'mc> for "+name+"<'mc> {")
        file_cache[mod_path].append("    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {")
        file_cache[mod_path].append("        self.0.clone()")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("    ")
        file_cache[mod_path].append("    fn jni_object(&self) -> jni::objects::JObject<'mc> {")
        file_cache[mod_path].append("        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("}")
        #file_cache[mod_path].append("impl Drop for "+name+"<'_> {")
        #file_cache[mod_path].append("    fn drop(&mut self) {")
        #file_cache[mod_path].append("        std::mem::forget(&self.2)")
        #file_cache[mod_path].append("    }")
        #file_cache[mod_path].append("}")

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library, cl, classes)

        if "methods" in val:
            parse_methods(library, name,val["methods"],mod_path,True,False,False,variants,False)

    elif val["isInterface"]: # interface generation
        file_cache[mod_path].append(
            "/// An instantiatable struct that implements "+name+". Needed for returning it from Java."
        )
        file_cache[mod_path].append(
            "pub struct "+name+"<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);"
        )
        file_cache[mod_path].append(
            "impl<'mc> "+name+"<'mc> {")
        if "methods" in val:
            parse_methods(library,name,val["methods"],mod_path,False,True,True,[],False)
        file_cache[mod_path].append("}")

        file_cache[mod_path].append("impl<'mc> crate::JNIRaw<'mc> for "+name+"<'mc> {")
        file_cache[mod_path].append("    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {")
        file_cache[mod_path].append("        self.0.clone()")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("    ")
        file_cache[mod_path].append("    fn jni_object(&self) -> jni::objects::JObject<'mc> {")
        file_cache[mod_path].append("        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("}")
        #if "interfaces" in val:
        #    for inter in val["interfaces"]:
        #        print(inter["name"])
    else:  # struct generation
        file_cache[mod_path].append(
            "pub struct "+name+"<'mc>(pub(crate) crate::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);"
        )

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library,cl, classes)

        file_cache[mod_path].append("impl<'mc> crate::JNIRaw<'mc> for "+name+"<'mc> {")
        file_cache[mod_path].append("    fn jni_ref(&self) -> crate::SharedJNIEnv<'mc> {")
        file_cache[mod_path].append("        self.0.clone()")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("    ")
        file_cache[mod_path].append("    fn jni_object(&self) -> jni::objects::JObject<'mc> {")
        file_cache[mod_path].append("        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("}")

        #if "constructors" in val:
        #    parse_methods(library, name,val["constructors"],mod_path,False,False,False,[],True)

        if "methods" in val:
            parse_methods(library,name,val["methods"],mod_path,False,False,False,[],False)


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

for library in libraries:
    packages = libraries[library]

    crate_dir = ""

    if library in library_resolves:
        crate_dir = os.path.join("src", library_resolves[library])
    else:
        print("Unhandled library "+library +
            ". All libraries must have corresponding rust crates.")
        exit(0)

    # delete and recreate the appropriate directory if it's the first time writing to it
    if crate_dir not in filled_once:
        shutil.rmtree(crate_dir)
        path = library.replace(".", os.sep)
        pathlib.Path(crate_dir+os.sep +
                    path).mkdir(parents=True, exist_ok=True)
    else:
        filled_once.push(crate_dir)

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

    mod_rs = crate_dir+os.sep+"mod.rs"

    if mod_rs not in file_cache:
        file_cache[mod_rs] = []

    with open(mod_rs, "a+") as file:
        for v in file_cache[mod_rs]:
            file.write(v+"\n")
        file.close()

    file_cache = {}

mod_rs_folder_populate("src")
os.rename("src"+os.sep+"mod.rs", "src"+os.sep+"lib.rs")

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
    os.system("rustfmt --unstable-features --skip-children ./src/bukkit/*"+wildcard+".rs")
    os.system("rustfmt --unstable-features --skip-children ./src/bungee/*"+wildcard+".rs")

os.system("cargo fix --allow-dirty --allow-staged --broken-code --jobs "+str(multiprocessing.cpu_count()))