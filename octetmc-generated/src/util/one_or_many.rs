use serde::{
    Deserializer as Deserer,
    Deserialize as Deser
};


#[derive(Deser)]
#[serde(untagged)]
enum OneOrMany<T> {
    One(T),
    Many(Vec<T>)
}


pub fn one_or_many<'de, D, T>(deserer : D) -> Result<Vec<T>, D::Error>
where
    D : Deserer<'de>,
    T : Deser<'de>
{
    match (OneOrMany::<T>::deserialize(deserer)?) {
        OneOrMany::One(v)  => Ok(vec![v]),
        OneOrMany::Many(v) => Ok(v),
    }
}
