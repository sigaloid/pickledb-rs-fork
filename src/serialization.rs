mod imports {
    #[cfg(feature = "nano")]
    pub use nanoserde::{DeBin, SerBin};
    #[cfg(not(feature = "nano"))]
    pub use serde::{de::DeserializeOwned, Serialize};
    pub use std::collections::HashMap;
    pub use std::fmt;
    pub use std::ops::Deref;

    // #[cfg(not(feature = "nano"))]
    pub type DbMap = HashMap<String, Vec<u8>>;
    // #[cfg(not(feature = "nano"))]
    pub type DbListMap = HashMap<String, Vec<Vec<u8>>>;

    #[cfg(feature = "nano")]
    mod hidden_types {
        use super::*;

        #[derive(DeBin, SerBin)]
        pub struct SerializedDB(pub Vec<u8>, pub Vec<u8>);
    }
    #[cfg(feature = "nano")]
    pub use hidden_types::SerializedDB;
}

use imports::*;

/// An enum for specifying the serialization method to use when creating a new PickleDB database
/// or loading one from a file
#[derive(Debug, Clone, Copy)]
pub enum SerializationMethod {
    /// [JSON serialization](https://crates.io/crates/serde_json)
    #[cfg(any(feature = "json"))]
    Json = 0,

    /// [Bincode serialization](https://crates.io/crates/bincode)
    #[cfg(any(feature = "bincode", feature = "nano"))]
    Bin = 1,

    /// [YAML serialization](https://crates.io/crates/serde_yaml)
    #[cfg(any(feature = "yaml"))]
    Yaml = 2,

    /// [CBOR serialization](https://crates.io/crates/serde_cbor)
    #[cfg(any(feature = "cbor"))]
    Cbor = 3,
}

#[allow(unreachable_patterns)]
impl From<i32> for SerializationMethod {
    fn from(item: i32) -> Self {
        match item {
            #[cfg(any(feature = "json"))]
            0 => SerializationMethod::Json,
            #[cfg(any(feature = "bincode", feature = "nano"))]
            1 => SerializationMethod::Bin,
            #[cfg(any(feature = "yaml"))]
            2 => SerializationMethod::Yaml,
            #[cfg(any(feature = "cbor"))]
            3 => SerializationMethod::Cbor,
            #[cfg(any(feature = "json"))]
            _ => SerializationMethod::Json,
            #[cfg(all(not(feature = "json"), any(feature = "bincode", feature = "nano")))]
            _ => SerializationMethod::Bin,
            #[cfg(all(not(feature = "json"), feature = "yaml"))]
            _ => SerializationMethod::Yaml,
            #[cfg(all(not(feature = "json"), feature = "cbor"))]
            _ => SerializationMethod::Cbor,
        }
    }
}

impl fmt::Display for SerializationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "json")]
struct JsonSerializer {}

#[cfg(feature = "json")]
impl JsonSerializer {
    fn new() -> JsonSerializer {
        JsonSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match serde_json::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match serde_json::to_string(data) {
            Ok(ser_data) => Ok(ser_data.into_bytes()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn serialize_db(&self, map: &DbMap, list_map: &DbListMap) -> Result<Vec<u8>, String> {
        let mut json_map: HashMap<&str, &str> = HashMap::new();
        for (key, value) in map.iter() {
            json_map.insert(key, std::str::from_utf8(value).unwrap());
        }

        let mut json_list_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (key, list) in list_map.iter() {
            let json_list: Vec<&str> = list
                .iter()
                .map(|item| std::str::from_utf8(item).unwrap())
                .collect();
            json_list_map.insert(key, json_list);
        }

        match serde_json::to_string(&(json_map, json_list_map)) {
            Ok(ser_db) => Ok(ser_db.into_bytes()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbListMap), String> {
        match serde_json::from_str::<(HashMap<String, String>, HashMap<String, Vec<String>>)>(
            std::str::from_utf8(ser_db).unwrap(),
        ) {
            Ok((json_map, json_list_map)) => {
                let mut byte_map: DbMap = HashMap::new();
                for (key, value) in json_map.iter() {
                    byte_map.insert(key.to_string(), value.as_bytes().to_vec());
                }

                let mut byte_list_map: DbListMap = HashMap::new();
                for (key, list) in json_list_map.iter() {
                    let byte_list: Vec<Vec<u8>> =
                        list.iter().map(|item| item.as_bytes().to_vec()).collect();
                    byte_list_map.insert(key.to_string(), byte_list);
                }

                Ok((byte_map, byte_list_map))
            }

            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(feature = "yaml")]
struct YamlSerializer {}

#[cfg(feature = "yaml")]
impl YamlSerializer {
    fn new() -> YamlSerializer {
        YamlSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match serde_yaml::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match serde_yaml::to_string(data) {
            Ok(ser_data) => Ok(ser_data.into_bytes()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn serialize_db(&self, map: &DbMap, list_map: &DbListMap) -> Result<Vec<u8>, String> {
        let mut yaml_map: HashMap<&str, &str> = HashMap::new();
        for (key, value) in map.iter() {
            yaml_map.insert(key, std::str::from_utf8(value).unwrap());
        }

        let mut yaml_list_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (key, list) in list_map.iter() {
            let yaml_list: Vec<&str> = list
                .iter()
                .map(|item| std::str::from_utf8(item).unwrap())
                .collect();
            yaml_list_map.insert(key, yaml_list);
        }

        match serde_yaml::to_string(&(yaml_map, yaml_list_map)) {
            Ok(ser_db) => Ok(ser_db.into_bytes()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbListMap), String> {
        match serde_yaml::from_str::<(HashMap<String, String>, HashMap<String, Vec<String>>)>(
            std::str::from_utf8(ser_db).unwrap(),
        ) {
            Ok((yaml_map, yaml_list_map)) => {
                let mut byte_map: DbMap = HashMap::new();
                for (key, value) in yaml_map.iter() {
                    byte_map.insert(key.to_string(), value.as_bytes().to_vec());
                }

                let mut byte_list_map: DbListMap = HashMap::new();
                for (key, list) in yaml_list_map.iter() {
                    let byte_list: Vec<Vec<u8>> =
                        list.iter().map(|item| item.as_bytes().to_vec()).collect();
                    byte_list_map.insert(key.to_string(), byte_list);
                }

                Ok((byte_map, byte_list_map))
            }

            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(feature = "bincode")]
struct BincodeSerializer {}

#[cfg(feature = "bincode")]
impl BincodeSerializer {
    fn new() -> BincodeSerializer {
        BincodeSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match bincode::deserialize(ser_data) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match bincode::serialize(data) {
            Ok(ser_data) => Ok(ser_data),
            Err(err) => Err(err.to_string()),
        }
    }

    fn serialize_db(&self, map: &DbMap, list_map: &DbListMap) -> Result<Vec<u8>, String> {
        self.serialize_data(&(map, list_map))
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbListMap), String> {
        match self.deserialize_data(ser_db) {
            Some((map, list_map)) => Ok((map, list_map)),
            None => Err(String::from("Cannot deserialize DB")),
        }
    }
}

#[cfg(feature = "nano")]
struct NanoBinSerializer {}

#[cfg(feature = "nano")]
impl NanoBinSerializer {
    fn new() -> NanoBinSerializer {
        NanoBinSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeBin,
    {
        match DeBin::deserialize_bin(ser_data) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: SerBin,
    {
        Ok(SerBin::serialize_bin(data))
    }

    fn serialize_db(&self, map: &DbMap, list_map: &DbListMap) -> Result<Vec<u8>, String> {
        let ser_db: SerializedDB =
            SerializedDB(SerBin::serialize_bin(map), SerBin::serialize_bin(list_map));
        self.serialize_data(&ser_db)
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbListMap), String> {
        match self.deserialize_data(ser_db) {
            Some(SerializedDB(map, list_map)) => {
                let map: HashMap<String, Vec<u8>> = match DeBin::deserialize_bin(&map) {
                    Ok(map) => map,
                    Err(_) => {
                        return Err(String::from("Cannot deserialize DB"));
                    }
                };
                let list_map: HashMap<String, Vec<Vec<u8>>> =
                    match DeBin::deserialize_bin(&list_map) {
                        Ok(map) => map,
                        Err(_) => {
                            return Err(String::from("Cannot deserialize DB"));
                        }
                    };
                Ok((map, list_map))
            }
            None => Err(String::from("Cannot deserialize DB")),
        }
    }
}

#[cfg(all(feature = "serde", feature = "cbor"))]
struct CborSerializer {}

#[cfg(all(feature = "serde", feature = "cbor"))]
impl CborSerializer {
    fn new() -> CborSerializer {
        CborSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match serde_cbor::from_slice(ser_data) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match serde_cbor::to_vec(data) {
            Ok(ser_data) => Ok(ser_data),
            Err(err) => Err(err.to_string()),
        }
    }

    fn serialize_db(&self, map: &DbMap, list_map: &DbListMap) -> Result<Vec<u8>, String> {
        self.serialize_data(&(map, list_map))
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbListMap), String> {
        match self.deserialize_data(ser_db) {
            Some((map, list_map)) => Ok((map, list_map)),
            None => Err(String::from("Cannot deserialize DB")),
        }
    }
}

pub(crate) struct Serializer {
    ser_method: SerializationMethod,
    #[cfg(feature = "json")]
    json_serializer: JsonSerializer,
    #[cfg(feature = "bincode")]
    bincode_serializer: BincodeSerializer,
    #[cfg(feature = "yaml")]
    yaml_serializer: YamlSerializer,
    #[cfg(feature = "cbor")]
    cbor_serializer: CborSerializer,
    #[cfg(feature = "nano")]
    nanoserdebin_serializer: NanoBinSerializer,
}

impl Serializer {
    pub(crate) fn new(ser_method: SerializationMethod) -> Serializer {
        Serializer {
            ser_method,
            #[cfg(feature = "json")]
            json_serializer: JsonSerializer::new(),
            #[cfg(feature = "bincode")]
            bincode_serializer: BincodeSerializer::new(),
            #[cfg(feature = "yaml")]
            yaml_serializer: YamlSerializer::new(),
            #[cfg(feature = "cbor")]
            cbor_serializer: CborSerializer::new(),
            #[cfg(feature = "nano")]
            nanoserdebin_serializer: NanoBinSerializer::new(),
        }
    }

    #[cfg(not(feature = "nano"))]
    pub(crate) fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.deserialize_data(ser_data),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bincode_serializer.deserialize_data(ser_data),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.deserialize_data(ser_data),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.deserialize_data(ser_data),
        }
    }

    #[cfg(feature = "nano")]
    pub(crate) fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeBin,
    {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            SerializationMethod::Bin => self.nanoserdebin_serializer.deserialize_data(ser_data),
            _ => self.nanoserdebin_serializer.deserialize_data(ser_data),
        }
    }

    #[cfg(not(feature = "nano"))]
    pub(crate) fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.serialize_data(data),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bincode_serializer.serialize_data(data),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.serialize_data(data),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.serialize_data(data),
        }
    }

    #[cfg(feature = "nano")]
    pub(crate) fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: SerBin,
    {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            SerializationMethod::Bin => self.nanoserdebin_serializer.serialize_data(data),
            _ => self.nanoserdebin_serializer.serialize_data(data),
        }
    }

    pub(crate) fn serialize_db(
        &self,
        map: &DbMap,
        list_map: &DbListMap,
    ) -> Result<Vec<u8>, String> {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.serialize_db(map, list_map),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bincode_serializer.serialize_db(map, list_map),
            #[cfg(feature = "nano")]
            SerializationMethod::Bin => self.nanoserdebin_serializer.serialize_db(map, list_map),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.serialize_db(map, list_map),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.serialize_db(map, list_map),
            #[cfg(test)]
            _ => Err(String::from("No serializer found")),
        }
    }

    pub(crate) fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbListMap), String> {
        #[allow(unreachable_patterns)]
        match self.ser_method {
            #[cfg(feature = "json")]
            SerializationMethod::Json => self.json_serializer.deserialize_db(ser_db),
            #[cfg(feature = "bincode")]
            SerializationMethod::Bin => self.bincode_serializer.deserialize_db(ser_db),
            #[cfg(feature = "nano")]
            SerializationMethod::Bin => self.nanoserdebin_serializer.deserialize_db(ser_db),
            #[cfg(feature = "yaml")]
            SerializationMethod::Yaml => self.yaml_serializer.deserialize_db(ser_db),
            #[cfg(feature = "cbor")]
            SerializationMethod::Cbor => self.cbor_serializer.deserialize_db(ser_db),
        }
    }
}
