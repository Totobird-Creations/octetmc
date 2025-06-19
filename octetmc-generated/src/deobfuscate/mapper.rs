use std::collections::HashMap;
use proguard::{ self, ProguardRecord };


pub struct ProguardMapper<'l> {
    pub classes : HashMap<&'l str, ProguardMapperClass<'l>>
}

pub struct ProguardMapperClass<'l> {
    pub original   : &'l str,
    pub obfuscated : &'l str,
    pub fields     : HashMap<&'l str, ProguardMapperField<'l>>,
    pub methods    : HashMap<&'l str, ProguardMapperMethod<'l>>
}

pub struct ProguardMapperField<'l> {
    pub original   : &'l str,
    pub obfuscated : &'l str,
    pub ty         : &'l str
}

pub struct ProguardMapperMethod<'l> {
    pub original   : &'l str,
    pub obfuscated : &'l str,
    pub ty         : &'l str,
    pub args       : &'l str
}


impl<'l> From<&'l proguard::ProguardMapping<'l>> for ProguardMapper<'l> {
    fn from(value : &'l proguard::ProguardMapping) -> Self {
        let mut latest_class = None;
        let mut classes      = HashMap::new();

        for record in value.iter() { match (record.unwrap()) {

            ProguardRecord::Header { .. } => { },

            ProguardRecord::Class { original, obfuscated } => {
                latest_class = Some(classes.entry(obfuscated).or_insert(ProguardMapperClass {
                    original,
                    obfuscated,
                    fields     : HashMap::new(),
                    methods    : HashMap::new()
                }));
            },

            ProguardRecord::Field { ty, original, obfuscated } => {
                latest_class.as_mut().unwrap().fields.insert(obfuscated, ProguardMapperField {
                    original,
                    obfuscated,
                    ty
                });
            },

            ProguardRecord::Method { ty, original, obfuscated, arguments, .. } => {
                latest_class.as_mut().unwrap().methods.insert(obfuscated, ProguardMapperMethod {
                    original,
                    obfuscated,
                    ty,
                    args       : arguments
                });
            }

        } }

        Self { classes }
    }
}
