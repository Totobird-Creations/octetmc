use std::io::{ Write, Read, Seek, Cursor };
use std::collections::HashMap;
use std::borrow::Cow;
use zip::ZipArchive;
use ristretto_classfile::{ ClassFile, Constant, FieldType, BaseType };
use ristretto_classfile::attributes::{ Attribute, Instruction, ArrayType };


mod mapper;
pub use mapper::*;


pub fn hash_file<H : Write, R : Read + Seek>(hasher : &mut H, archive : &mut ZipArchive<R>, mapper : &ProguardMapper, class_path : &str) {
    let class_path = class_path.replace(".", "/");
    if (class_path.starts_with("java/")
        || class_path.starts_with("javax/")
        || class_path.starts_with("jdk/jfr/")
        || class_path.starts_with("com/google/common/")
        || class_path.starts_with("io/netty/")
        || class_path.starts_with("it/unimi/dsi/fastutil/")
        || class_path.starts_with("com/mojang/authlib/")
        || class_path.starts_with("com/mojang/brigadier/")
        || class_path.starts_with("com/mojang/datafixers/")
        || class_path.starts_with("com/mojang/serialization/")
    ) { return; }
    let mut file_data = Vec::new();
    {
        let Some(mut file) = archive.index_for_name(&format!("{class_path}.class"))
            .and_then(|index| archive.by_index(index).ok())
            else { panic!("class {class_path:?} does not exist"); };
        file.read_to_end(&mut file_data).unwrap();
    }
    let class = ClassFile::from_bytes(&mut Cursor::new(file_data)).unwrap();
    hash_class(hasher, archive, mapper, &class)
}


pub fn hash_class<H : Write, R : Read + Seek>(hasher : &mut H, archive : &mut ZipArchive<R>, mapper : &ProguardMapper, class : &ClassFile) {
    let Some(Constant::Class(this_class)) = class.constant_pool.get(class.this_class)
        else { panic!() };
    let Some(Constant::Utf8(this_class)) = class.constant_pool.get(*this_class)
        else { panic!() };
    let fallback = ProguardMapperClass {
        original   : this_class,
        obfuscated : this_class,
        fields     : HashMap::new(),
        methods    : HashMap::new()
    };
    let class_deobf = mapper.classes.get(this_class.as_str()).unwrap_or(&fallback);

    hasher.write_all(&class.access_flags.bits().to_be_bytes()).unwrap();

    {
        hasher.write_all(class_deobf.original.as_bytes()).unwrap();
    }

    {
        let Some(Constant::Class(super_class)) = class.constant_pool.get(class.super_class)
            else { panic!() };
        let Some(Constant::Utf8(super_class)) = class.constant_pool.get(*super_class)
            else { panic!() };
        let super_class_deobf = mapper.classes.get(super_class.as_str()).map_or(super_class.as_str(), |super_class| super_class.original);
        hasher.write_all(super_class_deobf.as_bytes()).unwrap();
        hash_file(hasher, archive, mapper, super_class);
    }

    {
        let mut interfaces = class.interfaces.iter().map(|interface| {
            let Some(Constant::Class(interface)) = class.constant_pool.get(*interface)
                else { panic!(); };
            let Some(Constant::Utf8(interface)) = class.constant_pool.get(*interface)
                else { panic!(); };
            (interface, mapper.classes.get(interface.as_str()).map_or(interface.as_str(), |interface| interface.original),)
        }).collect::<Vec<_>>();
        interfaces.sort();
        for (interface, interface_deobf) in interfaces {
            hasher.write_all(interface_deobf.as_bytes()).unwrap();
            hash_file(hasher, archive, mapper, interface);
        }
    }

    {
        let mut fields = class.fields.iter().map(|field| {
            let Some(Constant::Utf8(name)) = class.constant_pool.get(field.name_index)
                else { panic!(); };
            let name_deobf = class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
            (name_deobf, field,)
        }).collect::<Vec<_>>();
        fields.sort_by(|(a, _,), (b, _,)| a.cmp(b));
        for (name, field,) in fields {

            hasher.write_all(&field.access_flags.bits().to_be_bytes()).unwrap();

            hasher.write_all(name.as_bytes()).unwrap();

            {
                let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(field.descriptor_index)
                    else { panic!(); };
                let descriptor = deobfuscate_type_desc(descriptor, mapper);
                hasher.write_all(descriptor.as_bytes()).unwrap();
            }

            {
                let mut checking_field_type = &field.field_type;
                loop { match (checking_field_type) {
                    FieldType::Base(t) => {
                        hasher.write_all(&[match (t) {
                            BaseType::Boolean => 0u8,
                            BaseType::Byte    => 1,
                            BaseType::Char    => 2,
                            BaseType::Double  => 3,
                            BaseType::Float   => 4,
                            BaseType::Int     => 5,
                            BaseType::Long    => 6,
                            BaseType::Short   => 7
                        }]).unwrap();
                        break;
                    },
                    FieldType::Object(object) => {
                        hasher.write_all(&[8]).unwrap();
                        let object_deobf = mapper.classes.get(object.as_str()).map_or(object.as_str(), |object| object.original);
                        hasher.write_all(object_deobf.as_bytes()).unwrap();
                        break;
                    },
                    FieldType::Array(f) => {
                        hasher.write_all(&[9]).unwrap();
                        checking_field_type = f;
                    },
                } }
            }

            hash_attributes(hasher, archive, mapper, class, &field.attributes);

        }
    }

    {
        let mut methods = class.methods.iter().map(|method| {
            let Some(Constant::Utf8(name)) = class.constant_pool.get(method.name_index)
                else { panic!(); };
            let name_deobf = class_deobf.methods.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
            (name_deobf, method,)
        }).collect::<Vec<_>>();
        methods.sort_by(|(a, _,), (b, _,)| a.cmp(b));
        for (name, method,) in methods {

            hasher.write_all(&method.access_flags.bits().to_be_bytes()).unwrap();

            hasher.write_all(name.as_bytes()).unwrap();

            {
                let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(method.descriptor_index)
                    else { panic!(); };
                let descriptor = deobfuscate_func_desc(descriptor, mapper);
                hasher.write_all(descriptor.as_bytes()).unwrap();
            }

            hash_attributes(hasher, archive, mapper, class, &method.attributes);

        }
    }

    hash_attributes(hasher, archive, mapper, class, &class.attributes);

}


fn deobfuscate_func_desc(descriptor : &str, mapper : &ProguardMapper) -> String {
    let Some(arb) = descriptor.strip_prefix('(')
        else { panic!(); };
    let (a, rb,) = arb.split_at(arb.chars().position(|ch| ch == ')').unwrap());
    let (_, b,)  = rb.split_at(1);
    let a = deobfuscate_type_desc(a, mapper);
    let b = deobfuscate_type_desc(b, mapper);
    format!("({a}){b}")
}

fn deobfuscate_type_desc(descriptor : &str, mapper : &ProguardMapper) -> String {
    descriptor.split(";").flat_map(|part| [
        Cow::Borrowed(";"),
        if let Some(part) = part.strip_prefix('L') {
            Cow::Owned(format!("L{}", mapper.classes.get(part).map_or(part, |part| part.original)))
        } else { Cow::Borrowed(part) }
    ]).skip(1).collect::<String>()
}


fn hash_attributes<H : Write, R : Read + Seek>(hasher : &mut H, archive : &mut ZipArchive<R>, mapper : &ProguardMapper, class : &ClassFile, attributes : &[Attribute]) {
    let mut attributes = attributes.iter().map(|attribute| {
        let Some(Constant::Utf8(name)) = class.constant_pool.get(attribute_name_index(attribute))
            else { panic!() };
        (name, attribute,)
    }).collect::<Vec<_>>();
    attributes.sort_by(|(a, _,), (b, _,)| a.cmp(b));
    for (name, attribute,) in attributes {

        match (attribute) {

            Attribute::ConstantValue { name_index : _, constant_value_index } => {

                hasher.write_all(name.as_bytes()).unwrap();

                let Some(value) = class.constant_pool.get(*constant_value_index)
                    else { panic!() };
                hash_constant(hasher, class, value);

            },

            Attribute::Code { name_index : _, max_stack, max_locals, code, exception_table, attributes } => {

                hasher.write_all(name.as_bytes()).unwrap();

                hasher.write_all(&max_stack.to_be_bytes()).unwrap();

                hasher.write_all(&max_locals.to_be_bytes()).unwrap();

                for instruction in code { match (instruction) {
                    Instruction::Nop                                          => { },
                    Instruction::Aconst_null                                  => hasher.write_all(&0u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_m1                                    => hasher.write_all(&1u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_0                                     => hasher.write_all(&2u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_1                                     => hasher.write_all(&3u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_2                                     => hasher.write_all(&4u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_3                                     => hasher.write_all(&5u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_4                                     => hasher.write_all(&6u8.to_be_bytes()).unwrap(),
                    Instruction::Iconst_5                                     => hasher.write_all(&7u8.to_be_bytes()).unwrap(),
                    Instruction::Lconst_0                                     => hasher.write_all(&8u8.to_be_bytes()).unwrap(),
                    Instruction::Lconst_1                                     => hasher.write_all(&9u8.to_be_bytes()).unwrap(),
                    Instruction::Fconst_0                                     => hasher.write_all(&10u8.to_be_bytes()).unwrap(),
                    Instruction::Fconst_1                                     => hasher.write_all(&11u8.to_be_bytes()).unwrap(),
                    Instruction::Fconst_2                                     => hasher.write_all(&12u8.to_be_bytes()).unwrap(),
                    Instruction::Dconst_0                                     => hasher.write_all(&13u8.to_be_bytes()).unwrap(),
                    Instruction::Dconst_1                                     => hasher.write_all(&14u8.to_be_bytes()).unwrap(),
                    Instruction::Bipush                                   (v) => {
                        hasher.write_all(&15u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Sipush                                   (v) => {
                        hasher.write_all(&16u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ldc                                      (v) => {
                        let Some(constant) = class.constant_pool.get(*v as u16)
                            else { panic!() };
                        hash_constant(hasher, class, constant);
                    },
                    Instruction::Ldc_w                                    (v) => {
                        let Some(constant) = class.constant_pool.get(*v)
                            else { panic!() };
                        hash_constant(hasher, class, constant);
                    },
                    Instruction::Ldc2_w                                   (v) => {
                        let Some(constant) = class.constant_pool.get(*v)
                            else { panic!() };
                        hash_constant(hasher, class, constant);
                    },
                    Instruction::Iload                                    (v) => {
                        hasher.write_all(&20u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Lload                                    (v) => {
                        hasher.write_all(&21u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Fload                                    (v) => {
                        hasher.write_all(&22u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Dload                                    (v) => {
                        hasher.write_all(&23u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Aload                                    (v) => {
                        hasher.write_all(&24u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Iload_0                                      => hasher.write_all(&25u8.to_be_bytes()).unwrap(),
                    Instruction::Iload_1                                      => hasher.write_all(&26u8.to_be_bytes()).unwrap(),
                    Instruction::Iload_2                                      => hasher.write_all(&27u8.to_be_bytes()).unwrap(),
                    Instruction::Iload_3                                      => hasher.write_all(&28u8.to_be_bytes()).unwrap(),
                    Instruction::Lload_0                                      => hasher.write_all(&29u8.to_be_bytes()).unwrap(),
                    Instruction::Lload_1                                      => hasher.write_all(&30u8.to_be_bytes()).unwrap(),
                    Instruction::Lload_2                                      => hasher.write_all(&31u8.to_be_bytes()).unwrap(),
                    Instruction::Lload_3                                      => hasher.write_all(&32u8.to_be_bytes()).unwrap(),
                    Instruction::Fload_0                                      => hasher.write_all(&33u8.to_be_bytes()).unwrap(),
                    Instruction::Fload_1                                      => hasher.write_all(&34u8.to_be_bytes()).unwrap(),
                    Instruction::Fload_2                                      => hasher.write_all(&35u8.to_be_bytes()).unwrap(),
                    Instruction::Fload_3                                      => hasher.write_all(&36u8.to_be_bytes()).unwrap(),
                    Instruction::Dload_0                                      => hasher.write_all(&37u8.to_be_bytes()).unwrap(),
                    Instruction::Dload_1                                      => hasher.write_all(&38u8.to_be_bytes()).unwrap(),
                    Instruction::Dload_2                                      => hasher.write_all(&39u8.to_be_bytes()).unwrap(),
                    Instruction::Dload_3                                      => hasher.write_all(&40u8.to_be_bytes()).unwrap(),
                    Instruction::Aload_0                                      => hasher.write_all(&41u8.to_be_bytes()).unwrap(),
                    Instruction::Aload_1                                      => hasher.write_all(&42u8.to_be_bytes()).unwrap(),
                    Instruction::Aload_2                                      => hasher.write_all(&43u8.to_be_bytes()).unwrap(),
                    Instruction::Aload_3                                      => hasher.write_all(&44u8.to_be_bytes()).unwrap(),
                    Instruction::Iaload                                       => hasher.write_all(&45u8.to_be_bytes()).unwrap(),
                    Instruction::Laload                                       => hasher.write_all(&46u8.to_be_bytes()).unwrap(),
                    Instruction::Faload                                       => hasher.write_all(&47u8.to_be_bytes()).unwrap(),
                    Instruction::Daload                                       => hasher.write_all(&48u8.to_be_bytes()).unwrap(),
                    Instruction::Aaload                                       => hasher.write_all(&49u8.to_be_bytes()).unwrap(),
                    Instruction::Baload                                       => hasher.write_all(&50u8.to_be_bytes()).unwrap(),
                    Instruction::Caload                                       => hasher.write_all(&51u8.to_be_bytes()).unwrap(),
                    Instruction::Saload                                       => hasher.write_all(&52u8.to_be_bytes()).unwrap(),
                    Instruction::Istore                                   (v) => {
                        hasher.write_all(&53u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Lstore                                   (v) => {
                        hasher.write_all(&54u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Fstore                                   (v) => {
                        hasher.write_all(&55u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Dstore                                   (v) => {
                        hasher.write_all(&56u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Astore                                   (v) => {
                        hasher.write_all(&57u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Istore_0                                     => hasher.write_all(&58u8.to_be_bytes()).unwrap(),
                    Instruction::Istore_1                                     => hasher.write_all(&59u8.to_be_bytes()).unwrap(),
                    Instruction::Istore_2                                     => hasher.write_all(&60u8.to_be_bytes()).unwrap(),
                    Instruction::Istore_3                                     => hasher.write_all(&61u8.to_be_bytes()).unwrap(),
                    Instruction::Lstore_0                                     => hasher.write_all(&62u8.to_be_bytes()).unwrap(),
                    Instruction::Lstore_1                                     => hasher.write_all(&63u8.to_be_bytes()).unwrap(),
                    Instruction::Lstore_2                                     => hasher.write_all(&64u8.to_be_bytes()).unwrap(),
                    Instruction::Lstore_3                                     => hasher.write_all(&65u8.to_be_bytes()).unwrap(),
                    Instruction::Fstore_0                                     => hasher.write_all(&66u8.to_be_bytes()).unwrap(),
                    Instruction::Fstore_1                                     => hasher.write_all(&67u8.to_be_bytes()).unwrap(),
                    Instruction::Fstore_2                                     => hasher.write_all(&68u8.to_be_bytes()).unwrap(),
                    Instruction::Fstore_3                                     => hasher.write_all(&69u8.to_be_bytes()).unwrap(),
                    Instruction::Dstore_0                                     => hasher.write_all(&70u8.to_be_bytes()).unwrap(),
                    Instruction::Dstore_1                                     => hasher.write_all(&71u8.to_be_bytes()).unwrap(),
                    Instruction::Dstore_2                                     => hasher.write_all(&72u8.to_be_bytes()).unwrap(),
                    Instruction::Dstore_3                                     => hasher.write_all(&73u8.to_be_bytes()).unwrap(),
                    Instruction::Astore_0                                     => hasher.write_all(&74u8.to_be_bytes()).unwrap(),
                    Instruction::Astore_1                                     => hasher.write_all(&75u8.to_be_bytes()).unwrap(),
                    Instruction::Astore_2                                     => hasher.write_all(&76u8.to_be_bytes()).unwrap(),
                    Instruction::Astore_3                                     => hasher.write_all(&77u8.to_be_bytes()).unwrap(),
                    Instruction::Iastore                                      => hasher.write_all(&78u8.to_be_bytes()).unwrap(),
                    Instruction::Lastore                                      => hasher.write_all(&79u8.to_be_bytes()).unwrap(),
                    Instruction::Fastore                                      => hasher.write_all(&80u8.to_be_bytes()).unwrap(),
                    Instruction::Dastore                                      => hasher.write_all(&81u8.to_be_bytes()).unwrap(),
                    Instruction::Aastore                                      => hasher.write_all(&82u8.to_be_bytes()).unwrap(),
                    Instruction::Bastore                                      => hasher.write_all(&83u8.to_be_bytes()).unwrap(),
                    Instruction::Castore                                      => hasher.write_all(&84u8.to_be_bytes()).unwrap(),
                    Instruction::Sastore                                      => hasher.write_all(&85u8.to_be_bytes()).unwrap(),
                    Instruction::Pop                                          => hasher.write_all(&86u8.to_be_bytes()).unwrap(),
                    Instruction::Pop2                                         => hasher.write_all(&87u8.to_be_bytes()).unwrap(),
                    Instruction::Dup                                          => hasher.write_all(&88u8.to_be_bytes()).unwrap(),
                    Instruction::Dup_x1                                       => hasher.write_all(&89u8.to_be_bytes()).unwrap(),
                    Instruction::Dup_x2                                       => hasher.write_all(&90u8.to_be_bytes()).unwrap(),
                    Instruction::Dup2                                         => hasher.write_all(&91u8.to_be_bytes()).unwrap(),
                    Instruction::Dup2_x1                                      => hasher.write_all(&92u8.to_be_bytes()).unwrap(),
                    Instruction::Dup2_x2                                      => hasher.write_all(&93u8.to_be_bytes()).unwrap(),
                    Instruction::Swap                                         => hasher.write_all(&94u8.to_be_bytes()).unwrap(),
                    Instruction::Iadd                                         => hasher.write_all(&95u8.to_be_bytes()).unwrap(),
                    Instruction::Ladd                                         => hasher.write_all(&96u8.to_be_bytes()).unwrap(),
                    Instruction::Fadd                                         => hasher.write_all(&97u8.to_be_bytes()).unwrap(),
                    Instruction::Dadd                                         => hasher.write_all(&98u8.to_be_bytes()).unwrap(),
                    Instruction::Isub                                         => hasher.write_all(&99u8.to_be_bytes()).unwrap(),
                    Instruction::Lsub                                         => hasher.write_all(&100u8.to_be_bytes()).unwrap(),
                    Instruction::Fsub                                         => hasher.write_all(&101u8.to_be_bytes()).unwrap(),
                    Instruction::Dsub                                         => hasher.write_all(&102u8.to_be_bytes()).unwrap(),
                    Instruction::Imul                                         => hasher.write_all(&103u8.to_be_bytes()).unwrap(),
                    Instruction::Lmul                                         => hasher.write_all(&104u8.to_be_bytes()).unwrap(),
                    Instruction::Fmul                                         => hasher.write_all(&105u8.to_be_bytes()).unwrap(),
                    Instruction::Dmul                                         => hasher.write_all(&106u8.to_be_bytes()).unwrap(),
                    Instruction::Idiv                                         => hasher.write_all(&107u8.to_be_bytes()).unwrap(),
                    Instruction::Ldiv                                         => hasher.write_all(&108u8.to_be_bytes()).unwrap(),
                    Instruction::Fdiv                                         => hasher.write_all(&109u8.to_be_bytes()).unwrap(),
                    Instruction::Ddiv                                         => hasher.write_all(&110u8.to_be_bytes()).unwrap(),
                    Instruction::Irem                                         => hasher.write_all(&111u8.to_be_bytes()).unwrap(),
                    Instruction::Lrem                                         => hasher.write_all(&112u8.to_be_bytes()).unwrap(),
                    Instruction::Frem                                         => hasher.write_all(&113u8.to_be_bytes()).unwrap(),
                    Instruction::Drem                                         => hasher.write_all(&114u8.to_be_bytes()).unwrap(),
                    Instruction::Ineg                                         => hasher.write_all(&115u8.to_be_bytes()).unwrap(),
                    Instruction::Lneg                                         => hasher.write_all(&116u8.to_be_bytes()).unwrap(),
                    Instruction::Fneg                                         => hasher.write_all(&117u8.to_be_bytes()).unwrap(),
                    Instruction::Dneg                                         => hasher.write_all(&118u8.to_be_bytes()).unwrap(),
                    Instruction::Ishl                                         => hasher.write_all(&119u8.to_be_bytes()).unwrap(),
                    Instruction::Lshl                                         => hasher.write_all(&120u8.to_be_bytes()).unwrap(),
                    Instruction::Ishr                                         => hasher.write_all(&121u8.to_be_bytes()).unwrap(),
                    Instruction::Lshr                                         => hasher.write_all(&122u8.to_be_bytes()).unwrap(),
                    Instruction::Iushr                                        => hasher.write_all(&123u8.to_be_bytes()).unwrap(),
                    Instruction::Lushr                                        => hasher.write_all(&124u8.to_be_bytes()).unwrap(),
                    Instruction::Iand                                         => hasher.write_all(&125u8.to_be_bytes()).unwrap(),
                    Instruction::Land                                         => hasher.write_all(&126u8.to_be_bytes()).unwrap(),
                    Instruction::Ior                                          => hasher.write_all(&127u8.to_be_bytes()).unwrap(),
                    Instruction::Lor                                          => hasher.write_all(&128u8.to_be_bytes()).unwrap(),
                    Instruction::Ixor                                         => hasher.write_all(&129u8.to_be_bytes()).unwrap(),
                    Instruction::Lxor                                         => hasher.write_all(&130u8.to_be_bytes()).unwrap(),
                    Instruction::Iinc                                  (v, c) => {
                        hasher.write_all(&131u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                        hasher.write_all(&c.to_be_bytes()).unwrap();
                    },
                    Instruction::I2l                                          => hasher.write_all(&132u8.to_be_bytes()).unwrap(),
                    Instruction::I2f                                          => hasher.write_all(&133u8.to_be_bytes()).unwrap(),
                    Instruction::I2d                                          => hasher.write_all(&134u8.to_be_bytes()).unwrap(),
                    Instruction::L2i                                          => hasher.write_all(&135u8.to_be_bytes()).unwrap(),
                    Instruction::L2f                                          => hasher.write_all(&136u8.to_be_bytes()).unwrap(),
                    Instruction::L2d                                          => hasher.write_all(&137u8.to_be_bytes()).unwrap(),
                    Instruction::F2i                                          => hasher.write_all(&138u8.to_be_bytes()).unwrap(),
                    Instruction::F2l                                          => hasher.write_all(&139u8.to_be_bytes()).unwrap(),
                    Instruction::F2d                                          => hasher.write_all(&140u8.to_be_bytes()).unwrap(),
                    Instruction::D2i                                          => hasher.write_all(&141u8.to_be_bytes()).unwrap(),
                    Instruction::D2l                                          => hasher.write_all(&142u8.to_be_bytes()).unwrap(),
                    Instruction::D2f                                          => hasher.write_all(&143u8.to_be_bytes()).unwrap(),
                    Instruction::I2b                                          => hasher.write_all(&144u8.to_be_bytes()).unwrap(),
                    Instruction::I2c                                          => hasher.write_all(&145u8.to_be_bytes()).unwrap(),
                    Instruction::I2s                                          => hasher.write_all(&146u8.to_be_bytes()).unwrap(),
                    Instruction::Lcmp                                         => hasher.write_all(&147u8.to_be_bytes()).unwrap(),
                    Instruction::Fcmpl                                        => hasher.write_all(&148u8.to_be_bytes()).unwrap(),
                    Instruction::Fcmpg                                        => hasher.write_all(&149u8.to_be_bytes()).unwrap(),
                    Instruction::Dcmpl                                        => hasher.write_all(&150u8.to_be_bytes()).unwrap(),
                    Instruction::Dcmpg                                        => hasher.write_all(&151u8.to_be_bytes()).unwrap(),
                    Instruction::Ifeq                                     (v) => {
                        hasher.write_all(&152u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ifne                                     (v) => {
                        hasher.write_all(&153u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Iflt                                     (v) => {
                        hasher.write_all(&154u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ifge                                     (v) => {
                        hasher.write_all(&155u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ifgt                                     (v) => {
                        hasher.write_all(&156u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ifle                                     (v) => {
                        hasher.write_all(&157u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_icmpeq                                (v) => {
                        hasher.write_all(&158u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_icmpne                                (v) => {
                        hasher.write_all(&159u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_icmplt                                (v) => {
                        hasher.write_all(&160u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_icmpge                                (v) => {
                        hasher.write_all(&161u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_icmpgt                                (v) => {
                        hasher.write_all(&162u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_icmple                                (v) => {
                        hasher.write_all(&163u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_acmpeq                                (v) => {
                        hasher.write_all(&164u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::If_acmpne                                (v) => {
                        hasher.write_all(&165u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Goto                                     (v) => {
                        hasher.write_all(&166u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Jsr                                      (v) => {
                        hasher.write_all(&167u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ret                                      (v) => {
                        hasher.write_all(&169u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Tableswitch  { default, low, high, offsets } => {
                        hasher.write_all(&170u8.to_be_bytes()).unwrap();
                        hasher.write_all(&default.to_be_bytes()).unwrap();
                        hasher.write_all(&low.to_be_bytes()).unwrap();
                        hasher.write_all(&high.to_be_bytes()).unwrap();
                        let mut offsets =  offsets.to_vec();
                        offsets.sort();
                        for offset in offsets {
                            hasher.write_all(&offset.to_be_bytes()).unwrap();
                        }
                    },
                    Instruction::Lookupswitch { default, pairs }              => {
                        hasher.write_all(&171u8.to_be_bytes()).unwrap();
                        hasher.write_all(&default.to_be_bytes()).unwrap();
                        let mut pairs = pairs.iter().collect::<Vec<_>>();
                        pairs.sort();
                        for (a, b,) in pairs {
                            hasher.write_all(&a.to_be_bytes()).unwrap();
                            hasher.write_all(&b.to_be_bytes()).unwrap();
                        }
                    },
                    Instruction::Ireturn                                      => hasher.write_all(&172u8.to_be_bytes()).unwrap(),
                    Instruction::Lreturn                                      => hasher.write_all(&173u8.to_be_bytes()).unwrap(),
                    Instruction::Freturn                                      => hasher.write_all(&174u8.to_be_bytes()).unwrap(),
                    Instruction::Dreturn                                      => hasher.write_all(&175u8.to_be_bytes()).unwrap(),
                    Instruction::Areturn                                      => hasher.write_all(&176u8.to_be_bytes()).unwrap(),
                    Instruction::Return                                       => hasher.write_all(&177u8.to_be_bytes()).unwrap(),
                    Instruction::Getstatic                                (v) => {
                        hasher.write_all(&178u8.to_be_bytes()).unwrap();
                        let Some(Constant::FieldRef { class_index : wrapping_class_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_type_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Putstatic                                (v) => {
                        hasher.write_all(&179u8.to_be_bytes()).unwrap();
                        let Some(Constant::FieldRef { class_index : wrapping_class_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_type_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Getfield                                 (v) => {
                        hasher.write_all(&180u8.to_be_bytes()).unwrap();
                        let Some(Constant::FieldRef { class_index : wrapping_class_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_type_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Putfield                                 (v) => {
                        hasher.write_all(&181u8.to_be_bytes()).unwrap();
                        let Some(Constant::FieldRef { class_index : wrapping_class_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_type_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Invokevirtual                            (v) => {
                        hasher.write_all(&182u8.to_be_bytes()).unwrap();
                        let Some(Constant::MethodRef { class_index : wrapping_class_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_func_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Invokespecial                            (v) => {
                        hasher.write_all(&183u8.to_be_bytes()).unwrap();
                        let Some(Constant::MethodRef { class_index : wrapping_class_index, name_and_type_index }
                            | Constant::InterfaceMethodRef { class_index : wrapping_class_index, name_and_type_index }
                        ) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_func_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Invokestatic                             (v) => {
                        hasher.write_all(&184u8.to_be_bytes()).unwrap();
                        let Some(Constant::MethodRef { class_index : wrapping_class_index, name_and_type_index }
                            | Constant::InterfaceMethodRef { class_index : wrapping_class_index, name_and_type_index }
                        ) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_func_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::Invokeinterface                       (v, c) => {
                        hasher.write_all(&185u8.to_be_bytes()).unwrap();
                        let Some(Constant::InterfaceMethodRef { class_index : wrapping_class_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*wrapping_class_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        let name_deobf = wrapping_class_deobf.fields.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original);
                        hasher.write_all(name_deobf.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_func_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                        hasher.write_all(&c.to_be_bytes()).unwrap();
                    },
                    Instruction::Invokedynamic                            (v) => {
                        hasher.write_all(&186u8.to_be_bytes()).unwrap();
                        let Some(Constant::InvokeDynamic { bootstrap_method_attr_index, name_and_type_index }) = class.constant_pool.get(*v)
                            else { panic!(); };
                        hasher.write_all(&bootstrap_method_attr_index.to_be_bytes()).unwrap();
                        let Some(Constant::NameAndType { name_index, descriptor_index }) = class.constant_pool.get(*name_and_type_index)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        hasher.write_all(name.as_bytes()).unwrap();
                        let Some(Constant::Utf8(descriptor)) = class.constant_pool.get(*descriptor_index)
                            else { panic!(); };
                        let descriptor_deobf = deobfuscate_func_desc(descriptor, mapper);
                        hasher.write_all(descriptor_deobf.as_bytes()).unwrap();
                    },
                    Instruction::New                                      (v) => {
                        hasher.write_all(&187u8.to_be_bytes()).unwrap();
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        hasher.write_all(wrapping_class_deobf.original.as_bytes()).unwrap();
                    },
                    Instruction::Newarray                                 (v) => {
                        hasher.write_all(&188u8.to_be_bytes()).unwrap();
                        hasher.write_all(&match (v) {
                            ArrayType::Boolean => 0u8,
                            ArrayType::Char    => 1,
                            ArrayType::Float   => 2,
                            ArrayType::Double  => 3,
                            ArrayType::Byte    => 4,
                            ArrayType::Short   => 5,
                            ArrayType::Int     => 6,
                            ArrayType::Long    => 7
                        }.to_be_bytes()).unwrap();
                    },
                    Instruction::Anewarray                                (v) => {
                        hasher.write_all(&189u8.to_be_bytes()).unwrap();
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        hasher.write_all(wrapping_class_deobf.original.as_bytes()).unwrap();
                    },
                    Instruction::Arraylength                                  => hasher.write_all(&190u8.to_be_bytes()).unwrap(),
                    Instruction::Athrow                                       => hasher.write_all(&191u8.to_be_bytes()).unwrap(),
                    Instruction::Checkcast                                (v) => {
                        hasher.write_all(&192u8.to_be_bytes()).unwrap();
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        hasher.write_all(wrapping_class_deobf.original.as_bytes()).unwrap();
                    },
                    Instruction::Instanceof                               (v) => {
                        hasher.write_all(&193u8.to_be_bytes()).unwrap();
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        hasher.write_all(wrapping_class_deobf.original.as_bytes()).unwrap();
                    },
                    Instruction::Monitorenter                                 => hasher.write_all(&194u8.to_be_bytes()).unwrap(),
                    Instruction::Monitorexit                                  => hasher.write_all(&195u8.to_be_bytes()).unwrap(),
                    Instruction::Wide                                         => hasher.write_all(&196u8.to_be_bytes()).unwrap(),
                    Instruction::Multianewarray                        (v, c) => {
                        hasher.write_all(&197u8.to_be_bytes()).unwrap();
                        let Some(Constant::Class(wrapping_class)) = class.constant_pool.get(*v)
                            else { panic!(); };
                        let Some(Constant::Utf8(wrapping_class)) = class.constant_pool.get(*wrapping_class)
                            else { panic!(); };
                        let fallback = ProguardMapperClass {
                            original   : wrapping_class,
                            obfuscated : wrapping_class,
                            fields     : HashMap::new(),
                            methods    : HashMap::new()
                        };
                        let wrapping_class_deobf = mapper.classes.get(wrapping_class.as_str()).unwrap_or(&fallback);
                        hasher.write_all(wrapping_class_deobf.original.as_bytes()).unwrap();
                        hasher.write_all(&c.to_be_bytes()).unwrap();
                    },
                    Instruction::Ifnull                                   (v) => {
                        hasher.write_all(&198u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Ifnonnull                                (v) => {
                        hasher.write_all(&199u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Goto_w                                   (v) => {
                        hasher.write_all(&200u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Jsr_w                                    (v) => {
                        hasher.write_all(&201u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Breakpoint                                   => hasher.write_all(&202u8.to_be_bytes()).unwrap(),
                    Instruction::Impdep1                                      => hasher.write_all(&203u8.to_be_bytes()).unwrap(),
                    Instruction::Impdep2                                      => hasher.write_all(&204u8.to_be_bytes()).unwrap(),
                    Instruction::Iload_w                                  (v) => {
                        hasher.write_all(&205u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Lload_w                                  (v) => {
                        hasher.write_all(&206u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Fload_w                                  (v) => {
                        hasher.write_all(&207u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Dload_w                                  (v) => {
                        hasher.write_all(&208u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Aload_w                                  (v) => {
                        hasher.write_all(&209u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Istore_w                                 (v) => {
                        hasher.write_all(&210u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Lstore_w                                 (v) => {
                        hasher.write_all(&211u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Fstore_w                                 (v) => {
                        hasher.write_all(&212u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Dstore_w                                 (v) => {
                        hasher.write_all(&213u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Astore_w                                 (v) => {
                        hasher.write_all(&214u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                    Instruction::Iinc_w                                (v, c) => {
                        hasher.write_all(&215u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                        hasher.write_all(&c.to_be_bytes()).unwrap();
                    },
                    Instruction::Ret_w                                    (v) => {
                        hasher.write_all(&216u8.to_be_bytes()).unwrap();
                        hasher.write_all(&v.to_be_bytes()).unwrap();
                    },
                } }

                let mut exceptions = exception_table.iter().map(|exception| {
                    let name_deobf = (if (exception.catch_type != 0) {
                        let Some(Constant::Class(name_index)) = class.constant_pool.get(exception.catch_type)
                            else { panic!(); };
                        let Some(Constant::Utf8(name)) = class.constant_pool.get(*name_index)
                            else { panic!(); };
                        Some(mapper.classes.get(name.as_str()).map_or(name.as_str(), |name_deobf| name_deobf.original))
                    } else { None } );
                    (name_deobf, exception,)
                }).collect::<Vec<_>>();
                exceptions.sort_by(|(a_name, a,), (b_name, b,)| a.range_pc.start.cmp(&b.range_pc.start).then(a.range_pc.end.cmp(&b.range_pc.end)).then(a.handler_pc.cmp(&b.handler_pc)).then(a_name.cmp(b_name)));
                for (name, exception,) in exceptions {
                    hasher.write_all(&exception.range_pc.start.to_be_bytes()).unwrap();
                    hasher.write_all(&exception.range_pc.end.to_be_bytes()).unwrap();
                    hasher.write_all(&exception.handler_pc.to_be_bytes()).unwrap();
                    if let Some(name) = name {
                        hasher.write_all(name.as_bytes()).unwrap();
                    }
                }

                hash_attributes(hasher, archive, mapper, class, attributes);

            },

            Attribute::StackMapTable { .. } => { },

            Attribute::Exceptions { .. } => { },

            Attribute::InnerClasses { .. } => { },

            Attribute::EnclosingMethod { .. } => { },

            Attribute::Synthetic { .. } => { },

            Attribute::Signature { .. } => { },

            Attribute::SourceFile { .. } => { },

            Attribute::SourceDebugExtension { .. } => { },

            Attribute::LineNumberTable { .. } => { },

            Attribute::LocalVariableTable { .. } => { },

            Attribute::LocalVariableTypeTable { .. } => { },

            Attribute::Deprecated { .. } => { },

            Attribute::RuntimeVisibleAnnotations { .. } => { },

            Attribute::RuntimeInvisibleAnnotations { .. } => { },

            Attribute::RuntimeVisibleParameterAnnotations { .. } => { },

            Attribute::RuntimeInvisibleParameterAnnotations { .. } => { },

            Attribute::RuntimeVisibleTypeAnnotations { .. } => { },

            Attribute::RuntimeInvisibleTypeAnnotations { .. } => { },

            Attribute::AnnotationDefault { .. } => { },

            Attribute::BootstrapMethods { .. } => { },

            Attribute::MethodParameters { .. } => { },

            Attribute::Module { .. } => { },

            Attribute::ModulePackages { .. } => { },

            Attribute::ModuleMainClass { .. } => { },

            Attribute::NestHost { .. } => { },

            Attribute::NestMembers { .. } => { },

            Attribute::Record { .. } => { },

            Attribute::PermittedSubclasses { .. } => { },

            Attribute::Unknown { .. } => { }
        }

    }
}


fn hash_constant<H : Write>(hasher : &mut H, class : &ClassFile, constant : &Constant) {
    match (constant) {
        Constant::Utf8               (v)    => {
            hasher.write_all(&0u8.to_be_bytes()).unwrap();
            hasher.write_all(v.as_bytes()).unwrap();
        },
        Constant::Integer            (v)    => {
            hasher.write_all(&1u8.to_be_bytes()).unwrap();
            hasher.write_all(&v.to_be_bytes()).unwrap();
        },
        Constant::Float              (v)    => {
            hasher.write_all(&2u8.to_be_bytes()).unwrap();
            hasher.write_all(&v.to_be_bytes()).unwrap();
        },
        Constant::Long               (v)    => {
            hasher.write_all(&3u8.to_be_bytes()).unwrap();
            hasher.write_all(&v.to_be_bytes()).unwrap();
        },
        Constant::Double             (v)    => {
            hasher.write_all(&4u8.to_be_bytes()).unwrap();
            hasher.write_all(&v.to_be_bytes()).unwrap();
        },
        Constant::Class              (i)    => {
            let Some(Constant::Utf8(v)) = class.constant_pool.get(*i)
                else { panic!(); };
            hasher.write_all(v.as_bytes()).unwrap();
        },
        Constant::String             (i)    =>{
            let Some(Constant::Utf8(v)) = class.constant_pool.get(*i)
                else { panic!(); };
            hasher.write_all(v.as_bytes()).unwrap();
        },
        Constant::FieldRef           { .. } => panic!(),
        Constant::MethodRef          { .. } => panic!(),
        Constant::InterfaceMethodRef { .. } => panic!(),
        Constant::NameAndType        { .. } => panic!(),
        Constant::MethodHandle       { .. } => panic!(),
        Constant::MethodType         (_)    => panic!(),
        Constant::Dynamic            { .. } => panic!(),
        Constant::InvokeDynamic      { .. } => panic!(),
        Constant::Module             (_)    => panic!(),
        Constant::Package            (_)    => panic!()
    }
}

fn attribute_name_index(attribute : &Attribute) -> u16 { *(match (attribute) {
    Attribute::ConstantValue                        { name_index, .. } => name_index,
    Attribute::Code                                 { name_index, .. } => name_index,
    Attribute::StackMapTable                        { name_index, .. } => name_index,
    Attribute::Exceptions                           { name_index, .. } => name_index,
    Attribute::InnerClasses                         { name_index, .. } => name_index,
    Attribute::EnclosingMethod                      { name_index, .. } => name_index,
    Attribute::Synthetic                            { name_index     } => name_index,
    Attribute::Signature                            { name_index, .. } => name_index,
    Attribute::SourceFile                           { name_index, .. } => name_index,
    Attribute::SourceDebugExtension                 { name_index, .. } => name_index,
    Attribute::LineNumberTable                      { name_index, .. } => name_index,
    Attribute::LocalVariableTable                   { name_index, .. } => name_index,
    Attribute::LocalVariableTypeTable               { name_index, .. } => name_index,
    Attribute::Deprecated                           { name_index     } => name_index,
    Attribute::RuntimeVisibleAnnotations            { name_index, .. } => name_index,
    Attribute::RuntimeInvisibleAnnotations          { name_index, .. } => name_index,
    Attribute::RuntimeVisibleParameterAnnotations   { name_index, .. } => name_index,
    Attribute::RuntimeInvisibleParameterAnnotations { name_index, .. } => name_index,
    Attribute::RuntimeVisibleTypeAnnotations        { name_index, .. } => name_index,
    Attribute::RuntimeInvisibleTypeAnnotations      { name_index, .. } => name_index,
    Attribute::AnnotationDefault                    { name_index, .. } => name_index,
    Attribute::BootstrapMethods                     { name_index, .. } => name_index,
    Attribute::MethodParameters                     { name_index, .. } => name_index,
    Attribute::Module                               { name_index, .. } => name_index,
    Attribute::ModulePackages                       { name_index, .. } => name_index,
    Attribute::ModuleMainClass                      { name_index, .. } => name_index,
    Attribute::NestHost                             { name_index, .. } => name_index,
    Attribute::NestMembers                          { name_index, .. } => name_index,
    Attribute::Record                               { name_index, .. } => name_index,
    Attribute::PermittedSubclasses                  { name_index, .. } => name_index,
    Attribute::Unknown                              { name_index, .. } => name_index
}) }
