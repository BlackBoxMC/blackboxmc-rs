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
                  "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield","await"]

library_resolves = {
    "net.md_5": "blackboxmc-rs-bungee",
    "org.bukkit": "blackboxmc-rs-bukkit",
    "java.util": "blackboxmc-rs-java",  # NOTE: written by hand.
    "java.lang": "blackboxmc-rs-java",  # NOTE: written by hand.
}

parsed_classes = {}
excluded_classes = [
    "org.bukkit.plugin.SimplePluginManager",    # uses stuff that isn't being generated due to that java bug and i don't want to write an entire class binding for something that's getting deprecated anyways.
    "java.lang.Thread",                         # i would have to bind all of java.lang for this and i don't feel like it sorry
    
    "java.util.concurrent",                     # i want to lessen my workload and binding this is going to be pointless since your only option of multithreading is through BukkitRunnables.
    "java.util.stream",                         # unneeded when java's iterator is bound.
    "java.util.function",                       # useless unless i can do implement Into for the functions in question and that can't be done safely.
]

interface_names = []
filled_once = []
bindings = {}
omitted_classes = ["Striped64"]
enums = []


def in_excluded_classes(cl):
    if cl in excluded_classes:
        return True
    for c in excluded_classes:
        if cl.startswith(c):
            return True
    return False

def library_name_format(libname):
    return libname.replace("-","_").replace("_rs_","_")

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

def func_signature_format(ty, increment, returning, options_start_at, generic_letters, upper_generic_letters,concrete_types):
    thing = ""
    internal_do_into = ty["type_name_resolved"].startswith("crate") or ty["type_name_resolved"].startswith("bukkit") or ty["type_name_resolved"].startswith("blackbox")
    is_string = ty["type_name_resolved"] == "String"
    internal = internal_do_into or ty["type_name_resolved"].startswith("jni")
    generic_letters = []
    generic_args = ty["generic_clauses"]

    generic_letters = list(filter(lambda f: f != "",generic_letters))
    concrete_types = list(filter(lambda f: f != "", concrete_types))

    generics = ty["generics"]
    generic_str = ""
    
    if(len(generics) < 1) and len(generic_letters) >= 1 and ty["type_name_original"] != "java.lang.Object":
        generics = generic_letters
    if(len(generics) < 1) and len(upper_generic_letters) == 1 and ty["type_name_original"] != "java.lang.Object":
        generics = upper_generic_letters
        
    if(len(generics) >= 1):
        generics_ = []
        n = 0
        for gen in generics:
            if len(concrete_types) < 1:
                gen2 = re.search("(extends|super) ([A-Za-z\s]*)",gen)
                if gen2 is not None:
                    gen = "Into<"+gen2.group(2)+">"
                if gen == "":
                    if len(upper_generic_letters) >= n:
                        generics_.append("/*1*/"+upper_generic_letters[n])
                    else:
                        generics_.append("/*2*/"+generic_letters[n])
                else:
                    generics_.append("/*3*/"+gen)
            n += 1
        generics = generics_

    generics += concrete_types
    generic_str = ", "+",".join(generics)

    if(len(generics) == 1 and ty["type_name_original"] == "java.lang.Object"):
        if not returning:
                thing += ty["type_name_lhand"]+": "
        thing += generics[len(generics)-1]
    else:
        if(len(generics) >= 1 and ty["type_name_original"] == "java.lang.Class"):
            generics = []
            generic_str = ""
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
                generic_letters.append(letter[0])
                generic_args.append(letter+": blackboxmc_general::JNIRaw<'mc> + "+old.replace("&dyn","")+"<'mc>")

            if ty["is_array"]:
                thing += "Vec<"
            if (internal_do_into or is_string) and not returning:
                thing += "impl Into<&'mc "
            if "<" in ty["type_name_resolved"] and "'mc" not in ty["type_name_resolved"]:
                ty["type_name_resolved"] = ty["type_name_resolved"].replace(">",",'mc>")
            thing += ty["type_name_resolved"]
            if(internal):
                if "'mc" not in ty["type_name_resolved"]:
                    thing += "<'mc"+generic_str+">"
            if (internal_do_into or is_string) and not returning:
                thing += ">"
            if ty["is_array"]:
                thing += ">"
            if(options_start_at != -1):
                if increment >= options_start_at:
                    thing += ">"
    while ",," in thing or ", ," in thing:
        thing = thing.replace(", ,",",").replace(",,",",")

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

    if options_start_at > -1:
        if n >= options_start_at-1:
            arg += ".unwrap()"

    ty = type["type_name_resolved"].replace("&dyn ","")

    if(ty.startswith("crate") or ty.startswith("blackbox")): # for internal types...
        # get the original object.
        fcall = ".into().jni_object().clone()"
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
                return ["let "+var_prefix+"_"+str(n)+" = jni::objects::JObject::from("+prefix+".new_string("+arg+".into()).unwrap());"]
            case "jni::objects::JObject" | "jni::objects::JClass":
                # nothing to do
                return ["let "+var_prefix+"_"+str(n)+" = "+arg+";"]
            case _:
                if len(type["type_name_resolved"]) <= 1:
                    return ["let "+var_prefix+"_"+str(n)+" = "+arg+".jni_object();"]
                else:
                    print("Untranslated argument:\t\t"+type["type_name_resolved"],"\t\t",type["type_name_original"])
                    return None

def return_format(return_group, prefix, static, method, obj_call, func_signature, types, is_trait, options_start_at, is_constructor=False):
    if return_group["is_array"]:
        return None
    else:
        end_line = "?"
    code = []
    if static:
        code.append("let cls = &jni.find_class(\""+return_group["type_name_original"].replace(".","/")+"\")"+end_line+";")
        if is_constructor:
            code.append("let res = "+prefix+".new_object(cls,")
        else:
            code.append("let res = "+prefix+".call_static_method(cls,\""+method["original_name"]+"\",")

        code.append("\""+java_call_signature_format(func_signature, return_group["type_name_original"],is_constructor)+"\",&["+
                    ",".join(types)+
                    "])"+end_line+";")
    else:
        code.append(
            "let res = "+prefix+".call_method("+
                    "&"+obj_call+","+
                    "\""+method["original_name"]+"\",\""+
                    java_call_signature_format(func_signature, return_group["type_name_original"])+"\",&["+
                    ",".join(types)+
                    "]);")
        if return_group["type_name_resolved"] != "()":
            code.append("let res = ")
        code.append(prefix+".translate_error(res)?;")

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

            if static:
                val_1 = "jni"
                if is_constructor:
                    val_2 = "res"
                else:
                    code.append("let mut obj = res.l()"+end_line+";")
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
            code.append(return_group["type_name_resolved"]+"::from_raw(&"+
                                                prefix+","+
                                                val_2)
            if return_group["type_name_original"] in enums:
                code.append(", "+return_group["type_name_resolved"]+"::from_string(variant_str).unwrap()")
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

def correct_extends_syntax(text):
    gen2 = re.findall("(extends|super) ([A-Za-z]*)",text)
    for gen in gen2:
        res = correct_extends_syntax(str(gen))
        if len(res) <= 1:
            text = text.replace(str(gen),res)
        else:
            text = text.replace(str(gen),"Into<"+res+">")
    return text
def java_type_to_rust(argname, ty, method, i, returning, library, is_constructor,is_trait, upper_generic_letters):
    type_name_resolved = ""
    is_array = False
    is_interface = False
    if ty.endswith("[]"):
        ty = ty.replace("[]", "")
        is_array = True

    type_alone = ""

    if ty.replace("com.destroystokyo.paper","org.bukkit") in interface_names:
        is_interface = True

    type_name_original = ty
    package_name = ".".join(filter(lambda f: f.islower(), type_name_original.split(".")))
    if in_excluded_classes(ty):
        return None

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
        case "java.lang.Object"  | "java.lang.reflect.Type" | "java.awt.Image" | "java.awt.image.BufferedImage":
            type_name_resolved = "jni::objects::JObject"
        case _:
            if len(ty) <= 2:
                type_name_resolved = ty
            else:
                if ty.startswith("java"):
                    tynamealone = "".join(filter(lambda f: f[0].upper() == f[0], ty.split(".")))
                    type_name_resolved = package_name+".Java"+tynamealone
                else:
                    type_name_resolved = ty

                crate_name = ".".join(
                    filter(lambda f: f.lower() == f, ty.split(".")))

                cont = True
                while crate_name not in library_resolves and crate_name != "":
                    parts = crate_name.split(".")
                    parts.pop()
                    crate_name = ".".join(parts)

                if crate_name == "":
                    # it just so happens that the current external/unbound functions are stuff we don't care to implement anyways. but we'll leave this here for the future.
                    #print("Unbound external class:",ty,"\t\t",method["method"]["name"])
                    return None
                else:
                    if crate_name == library or library is False:
                        to_replace = "crate"
                    else:
                        to_replace = library_name_format(library_resolves[crate_name])

                    type_name_resolved = type_name_resolved.replace(
                        crate_name, to_replace).replace(".", "::").replace("-", "_").replace("$", "")

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

    upper_generics_list = list(filter(lambda f: f != "",upper_generic_letters.replace(" ","").split(",")))
    generics = []
    generic_clauses = []
    if method is not None:
        if len(generics) <= 0:
            search = []
            if returning:
                if "genericReturnType" in method["method"]:
                    search = method["method"]["genericReturnType"].split(".")
            else:
                if "genericParameterTypes" in method["method"]:
                    search_ = method["method"]["genericParameterTypes"]
                    if len(search_) > i:
                        search = [search_[i]]
            if len(search) >= 1:
                n = i
                for f in search:
                    if in_excluded_classes(f):
                        return None
                    typ = java_type_to_rust("", f, None,n,returning,library,is_constructor,is_trait,upper_generic_letters)
                    if typ is not None:
                        if "::" in typ["type_name_resolved"] and typ["type_name_resolved"] != type_name_resolved:
                            gen = correct_extends_syntax(typ["type_name_resolved"])
                            # special case: some things return "Class<?>" but that's not supported
                            if ty != "java.lang.Class":
                                if gen == "javaString":
                                    gen = "String"
                                gen = gen.replace("? extends","").replace("? super","").replace("?","dyn JNIRaw<'mc>")
                                # another special case: if it's an "Object" then no it isn't, silly.
                                if ty == "java.lang.Object":
                                    generic_clauses.append(gen+": JNIRaw<'mc>")
                                    type_name_resolved = gen
                                    type_name_original = gen
                                else:
                                    if(len(gen) >= 1):
                                        type_name_resolved = gen
                                    else:
                                        generics.append("/* java_type_to_rust */"+gen)
                                        generic_clauses.append(gen+": Into<"+gen+">")
                    n += 1

                
    # if the generics array is empty, then we have a few rules.
    if len(generics) <= 0:
        # first we have to get the default generics to know some stuff.
        default_generics = []
        if ".".join(package_name.split(".")[0:2]) in libraries:
            temp_lib = libraries[".".join(package_name.split(".")[0:2])]
            if package_name in temp_lib:
                temp_pkg = temp_lib[package_name]
                typealone = "".join(filter(lambda f: f[0].isupper(), type_name_original.split(".")))
                if typealone.replace("$",".") in temp_pkg:
                    temp_cls = temp_pkg[typealone.replace("$",".")]
                    if "generics" in temp_cls:
                        default_generics = temp_cls["generics"]
        # is there only one upper generic, and one default generic?
        if len(upper_generics_list) == 1 and len(default_generics) == 1:
            generics = upper_generics_list
    
    if method is not None:
        # ....hey by any chance is the method a "public" method?
        if "isDefault" in method["method"]:
            default = method["method"]["isDefault"]
            if default:
                if(len(upper_generics_list) == len(generics)):
                    generics = upper_generics_list

    # by any change is the only generic in the type "T" and do we also only have one generic 
    if len(generics) == 1:
        if(generics[0] == "T"):
            if len(upper_generics_list) == 1:
                generics = upper_generics_list
    if type_alone == "":
        type_alone = type_name_resolved

    return {
        "type_name_resolved": type_name_resolved,
        "type_name_lhand": type_name_lhand,
        "is_array": is_array,
        "is_interface": is_interface,
        "package_name": package_name,
        "type_name_original": type_name_original,
        "type_name_alone": type_alone,
        "generics": generics,
        "generic_clauses": generic_clauses,
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

def gen_from_raw_func(name, is_enum, mod_path):
    impl_signature = []
    impl_signature.append("pub fn from_raw(env: &blackboxmc_general::SharedJNIEnv<'mc>, obj: jni::objects::JObject<'mc>")
    if is_enum:
        impl_signature.append(", e: "+name+"Enum")
    impl_signature.append(") -> Result<Self, Box<dyn std::error::Error>> {")
    impl_signature.append(
                "if obj.is_null() {\n"+
                "    return Err(eyre::eyre!(\n"+
                "        \"Tried to instantiate "+name+" from null object.\")\n"+
                "    .into());\n"+
                "}\n"+
                "let (valid, name) = env.validate_name(&obj, \""+name+"\")?;\n"+
                "if !valid {\n"+
                "    Err(eyre::eyre!(\n"+
                "        \"Invalid argument passed. Expected a "+name+" object, got {}\",\n"+
                "        name\n"+
                "    )\n"+
                "    .into())\n"+
                "} else {\n"+
                "    Ok(Self(env.clone(), obj")
    if is_enum:
        impl_signature.append(", e")
    impl_signature.append("))\n"+
                "}"
        )
    impl_signature.append("}")
    for impl in impl_signature:
        file_cache[mod_path].append(impl)

def parse_methods(library,name,methods,mod_path,is_enum,is_trait,is_trait_decl,variants,is_constructor,upper_generic_letters):
    og_name = name
    impl_signature = []

    if is_enum and not is_trait and not is_constructor:
        for (v,val_proper) in variants:
            impl_signature.append("pub const "+v.upper()+": "+name+"Enum = "+name+"Enum::"+val_proper+";")

        impl_signature.append("pub fn from_string(str: String) -> std::option::Option<"+name+"Enum> {\nmatch str.as_str() {")
        for (v,val_proper) in variants:
            impl_signature.append("\""+v+"\" => Some("+name+"Enum::"+val_proper+"),")
        impl_signature.append("_ => None}}")

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

                    m["options_start_at"] = options_start_at
                    method_buildup.append(m)

                    if not can_use_options:
                        breaking = True
                    else:
                        last_method = m
                    i += 1
                n += 1

                if n == len(methods__) and breaking is not True:
                    method_buildup.append(m)
                    breaking = True

                if breaking:
                    identifier = method_first_arg.split(".")[len(method_first_arg.split("."))-1].lower()
                    method_map[identifier] = method_buildup.copy()
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
        parameters = method["method"]["parameters"]
        generic_parameter_types = method["method"]["genericParameterTypes"]
        types = []
        for i in range(len(parameters)):
            x = parameters[i]
            y = generic_parameter_types[i]
            if(len(y) == 1 and x[1] == "java.lang.Object"):
                types.append([x[0], y])
            else:
                types.append(x)


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
                "generics": [],
                "generic_clauses": [],
                "package_name": "",
                "options_start_at": options_start_at,
            })
        else:
            func_signature.append({
                "type_name_resolved": "jni: blackboxmc_general::SharedJNIEnv<'mc>",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "type_name_original": "",
                "type_name_alone": "",
                "generics": [],
                "generic_clauses": [],
                "package_name": "",
                "options_start_at": options_start_at,
            })
        should_continue = True
        i = 0
        # make the function signature
        for type in types:
            group = java_type_to_rust(type[0], type[1],method, i, False, library,is_constructor,is_trait,upper_generic_letters)

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
            if(type["type_name_lhand"] == "") or (type["is_array"]) or in_excluded_classes(type["type_name_original"]):
                n += 1
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
        
        gen = None

        tyname = None
        if is_constructor:
            tyname = method["method"]["name"]
        else:
            tyname = method["method"]["returnType"]

        return_group = java_type_to_rust("", tyname, method, i, True, library, True, is_trait, upper_generic_letters)

        if return_group is None:
            continue
                
        if in_excluded_classes(return_group["type_name_original"]):
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
        m = return_format(return_group, prefix, static, method, obj_call, func_signature, types, is_trait,options_start_at, is_constructor)
        if m is None:
            continue
        code.append(m)

        func_signature_resolved = ""
        func_signature_resolved_parts = []
        generic_letters = []
        generic_args = []
        concrete_types = []

        j = 0
        for ty in func_signature:
            group = func_signature_format(ty,j,False,options_start_at,generic_letters,upper_generic_letters,concrete_types)

            if group is None:
                continue

            if group["result"] != "":
                func_signature_resolved_parts.append(group["result"])
                j += 1
            
            generic_letters += group["generic_letters"]
            generic_args += group["generic_args"]

        if "generics" in method["method"]:
            gen = get_generics(method["method"], False,upper_generic_letters)
            generic_letters += gen[0].split(",")
            generic_args += gen[1].split(",")
            concrete_types += gen[2]

        return_type = func_signature_format(return_group,j,True,-1,generic_letters,upper_generic_letters,concrete_types)
        generic_letters_str = ""

        if len(concrete_types) < 1 and len(generic_letters) >= 1:
            st = ",".join(generic_letters)
            if st != "":
                generic_letters_str = "<"+st+">"

        generic_args_str = ""
        if len(generic_args) >= 1:
            st = ",".join(generic_args)
            if st != "":
                generic_args_str = " where "+st

        func_signature_resolved = ",".join(func_signature_resolved_parts)

        if "annotations" in method["method"]:
            impl_signature += parse_annotations(method["method"]["annotations"])

        
        impl_signature.append(
            "\tpub fn "+name+generic_letters_str+"("+func_signature_resolved+") "
        )

        if return_type is None:
            continue

        impl_signature.append("-> Result<"+return_type["result"]+", Box<dyn std::error::Error>>")

        impl_signature.append(generic_args_str)

        if is_trait and not is_trait_decl:
            impl_signature.append(";")
        else:
            impl_signature.append("{"+"\n".join(code)+"}")

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

    if mod_path not in file_cache :
        file_cache[mod_path] = []

    if (name == ""):
        return

    # generics?
    generics_str_letters, generics_str_where, _ = get_generics(val,True,None)
    if generics_str_where != "":
        generics_str_where = " where "+generics_str_where

    if val["isEnum"]:  # enum generation
        file_cache[mod_path].append(
            "pub enum "+name+"Enum {")

        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            if "annotations" in val:
                if v in val["annotations"]:
                    file_cache[mod_path] += parse_annotations(val["annotations"][v])
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
            "pub struct "+name+"<'mc"+generics_str_letters+">(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>, pub "+name+"Enum)"+generics_str_where+";")

        file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> std::ops::Deref for "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")
        file_cache[mod_path].append("   type Target = "+name+"Enum;")
        file_cache[mod_path].append("   fn deref(&self) -> &Self::Target {")
        file_cache[mod_path].append("       return &self.2;")
        file_cache[mod_path].append("   }")
        file_cache[mod_path].append("}")
        file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> JNIRaw<'mc> for "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")
        file_cache[mod_path].append("    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {")
        file_cache[mod_path].append("        self.0.clone()")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("    ")
        file_cache[mod_path].append("    fn jni_object(&self) -> jni::objects::JObject<'mc> {")
        file_cache[mod_path].append("        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("}")

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library, cl, classes)

        file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")

        gen_from_raw_func(name, True, mod_path)

        if "methods" in val:
            parse_methods(library, name,val["methods"],mod_path,True,False,False,variants,False,generics_str_letters)

        file_cache[mod_path].append("}")

    elif val["isInterface"]: # interface generation
        file_cache[mod_path].append(
            "/// An instantiatable struct that implements "+name+". Needed for returning it from Java."
        )
        file_cache[mod_path].append(
            "pub struct "+name+"<'mc"+generics_str_letters+">(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>)"+generics_str_where+";"
        )
        file_cache[mod_path].append(
            "impl<'mc"+generics_str_letters+"> "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")

        gen_from_raw_func(name, False, mod_path)

        if "methods" in val:
            parse_methods(library,name,val["methods"],mod_path,False,True,True,[],False,generics_str_letters)
        file_cache[mod_path].append("}")

        file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> JNIRaw<'mc> for "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")
        file_cache[mod_path].append("    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {")
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
            "pub struct "+name+"<'mc"+generics_str_letters+">(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>)"+generics_str_where+";"
        )

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library,cl, classes)

        file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> blackboxmc_general::JNIRaw<'mc> for "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")
        file_cache[mod_path].append("    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {")
        file_cache[mod_path].append("        self.0.clone()")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("    ")
        file_cache[mod_path].append("    fn jni_object(&self) -> jni::objects::JObject<'mc> {")
        file_cache[mod_path].append("        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
        file_cache[mod_path].append("    }")
        file_cache[mod_path].append("}")

        file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> "+name+"<'mc"+generics_str_letters+">"+generics_str_where+" {")

        gen_from_raw_func(name, False, mod_path)

        if "constructors" in val:
            parse_methods(library, name,val["constructors"],mod_path,False,False,False,[],True,generics_str_letters)

        if "methods" in val:
            parse_methods(library,name,val["methods"],mod_path,False,False,False,[],False,generics_str_letters)

        file_cache[mod_path].append("}")

    if not val["isEnum"]:
        if "interfaces" in val:
            for inter in val["interfaces"]:
                parse_into_impl(inter,name,mod_path,generics_str_letters,generics_str_where)
        if "superClass" in val:
            super_class = val["superClass"]
            parse_into_impl(super_class,name,mod_path,generics_str_letters,generics_str_where)

def get_generics(val, mc_included,upper_generics):
    generics_letters = []
    generics_types = []
    generics_str_letters = ""
    generics_str_where = ""
    concrete_types = []
    if isinstance(val, str):
        if ".".join(val.split(".")[0:2]) in libraries:
            temp_lib = libraries[".".join(val.split(".")[0:2])]
            package_name = ".".join(filter(lambda f: f[0].islower(), val.split(".")))
            if package_name in temp_lib:
                temp_pkg = temp_lib[package_name]
                typealone = "".join(filter(lambda f: f[0].isupper(), val.split(".")))
                typealone = typealone.replace(".","$").replace("[]","")
                if typealone in temp_pkg:
                    temp_cls = temp_pkg[typealone]
                    if "generics" in temp_cls:
                        generics = temp_cls["generics"]
    else:
        if "generics" in val:
            generics = list(map(lambda f: (f[0], f[1:]), filter(lambda f: f != "", val["generics"])))
            if len(generics) >= 1:
                for f in generics:
                    if len(f[1]) <= 2:
                        generics_letters.append(f[0])
                        generics_types.append(f[0]+": JNIRaw<'mc>")
                    else:
                        typarts = f[1].split(" ")
                        tyname = typarts[len(typarts)-1]
                        if "extends" in f[1] or "super" in f[1]:
                            generics_letters.append(f[0])
                            generics_types.append(f[0]+": Into<crate::Java"+tyname+"<'mc>>")
                        else:
                            generics_letters.append(f[0]+f[1])
                            concrete_types.append(f[0]+f[1])
                generics_str_where = ",".join(generics_types)
            if mc_included:
                generics_str_letters += ", "
            generics_str_letters += ",".join(map(lambda f: f, generics_letters))
    if upper_generics is not None:
        if generics_str_letters == upper_generics.replace(" ","").replace(",",""):
            return "","",[]
    if upper_generics is not None:
        for g in generics_str_letters.split(","):
            if g in upper_generics:
                generics_str_letters = generics_str_letters.replace(g,"").replace(",,",",")
    return generics_str_letters, generics_str_where, concrete_types
def parse_into_impl(val,name,mod_path,generics_str_letters,generics_str_where):
    if val["packageName"].startswith("java"):
        if not val["packageName"].startswith("java.util"):
            return
    val_resolved = java_type_to_rust("", val["packageName"]+"."+val["name"], None, 0, True, library, False, False, generics_str_letters)

    if val_resolved is None:
        return
    if in_excluded_classes(val_resolved["type_name_original"]):
        return
    if val_resolved["type_name_alone"] in omitted_classes:
        return
    
    into_generics_str_letters = "/* parse_into_impl */"
    
    # we want to check if the generics are the same.
    if ".".join(val_resolved["package_name"].split(".")[0:2]) in libraries:
        temp_lib = libraries[".".join(val_resolved["package_name"].split(".")[0:2])]
        temp_pkg = temp_lib[val_resolved["package_name"]]
        typealone = "".join(filter(lambda f: f[0].isupper(), val_resolved["type_name_original"].split(".")))
        if typealone in temp_pkg:
            temp_cls = temp_pkg[typealone]
            if "generics" in temp_cls and "generics" in val_resolved:
                if(len(temp_cls["generics"]) != len(val_resolved["generics"])):
                    return
                m = list(filter(lambda f: f != "", generics_str_letters.split(",")))
                if(len(temp_cls["generics"]) != len(m)):
                    return
                similar = False not in map(lambda x, y: x == y, temp_cls["generics"], val_resolved["generics"])
                if not similar:
                    return
                else:
                    into_generics_str_letters += generics_str_letters
        
    file_cache[mod_path].append("impl<'mc"+generics_str_letters+"> Into<"+val_resolved["type_name_resolved"]+"<'mc"+into_generics_str_letters+">> for "+name+"<'mc"+generics_str_letters+">"+generics_str_where+"{\n"+
                            "   fn into(self) -> "+val_resolved["type_name_resolved"]+"<'mc"+into_generics_str_letters+"> {\n"+
                            "       "+val_resolved["type_name_resolved"]+"::from_raw(&self.jni_ref(), self.1).unwrap()\n"+
                            "   }\n"+
                            "}")

def parse_annotations(annotations):
    strings = []
    for annotation in annotations:
        match(annotation[0]):
            case "forRemoval":
                strings.append("#[deprecated]")
            case "since":
                nop = 0
            case _:
                print("unbound annotation",annotation[0])
    return strings
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
        #if library_resolves[library] == "blackboxmc-rs-java":
        #    continue 
        crate_dir = os.path.join(library_resolves[library], "src")
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
        filled_once.append(crate_dir)
    
    print("filled",filled_once,crate_dir)
        

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

    mod_rs_folder_populate(crate_dir)

    f1 = open(crate_dir+os.sep+"mod.rs", "r")
    f2 = open(crate_dir+os.sep+"lib.rs", "a")
    f3 = open(crate_dir+os.sep+"lib.rs", "r")

    lines = f3.readlines()
    if(len(lines) == 0):
        f2.write("#![allow(deprecated)]\nuse blackboxmc_general::JNIRaw;\nuse color_eyre::eyre::Result;")
    f2.write("".join(f1.readlines()))
    os.remove(crate_dir+os.sep+"mod.rs")

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
                try:
                    at = int(newlines.index("impl<'mc> "+append_to_struct+"<'mc> {\n"))
                    newlines.insert(at+1, lines)
                    f2.truncate(0)
                    f2.write("".join(newlines))
                except Exception as ex:
                    print(ex)
                    continue
            else:
                f2.write(lines)
            f2.close()
        f.close()

for i in range(0,4):
    wildcard = "*/*" * i
    for library in library_resolves:
        resolved = library_resolves[library]
        os.system("rustfmt --unstable-features --skip-children ./"+resolved+"/src/*"+wildcard+".rs")

os.system("cargo fix --allow-dirty --allow-staged --broken-code --jobs "+str(multiprocessing.cpu_count()))