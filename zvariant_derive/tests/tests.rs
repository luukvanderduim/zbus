#![allow(dead_code)]

use std::collections::HashMap;
use zvariant::{
    serialized::{Context, Format},
    DeserializeDict, OwnedValue, SerializeDict, Type, Value, LE,
};

#[test]
fn derive_unit_struct() {
    #[derive(Type)]
    struct FooF(f64);

    assert_eq!(FooF::SIGNATURE, "d")
}

#[test]
fn derive_struct() {
    #[derive(Type)]
    struct TestStruct {
        name: String,
        age: u8,
        blob: Vec<u8>,
    }

    assert_eq!(TestStruct::SIGNATURE, "(syay)")
}

#[test]
fn derive_enum() {
    #[repr(u32)]
    #[derive(Type)]
    enum RequestNameFlags {
        AllowReplacement = 0x01,
        ReplaceExisting = 0x02,
        DoNotQueue = 0x04,
    }

    assert_eq!(RequestNameFlags::SIGNATURE, "u")
}

#[test]
fn derive_dict() {
    #[derive(SerializeDict, DeserializeDict, Type)]
    #[zvariant(deny_unknown_fields, signature = "a{sv}", rename_all = "camelCase")]
    struct Test {
        field_a: Option<u32>,
        #[zvariant(rename = "field-b")]
        field_b: String,
        field_c: Vec<u8>,
    }

    let test = Test {
        field_a: Some(1),
        field_b: "foo".to_string(),
        field_c: vec![1, 2, 3],
    };

    let ctxt = Context::new(Format::DBus, LE, 0);
    let serialized = zvariant::to_bytes(ctxt, &test).unwrap();
    let deserialized: HashMap<String, OwnedValue> = serialized.deserialize().unwrap().0;

    assert_eq!(
        deserialized["fieldA"],
        Value::from(1u32).try_into().unwrap()
    );
    assert_eq!(
        deserialized["field-b"],
        Value::from("foo").try_into().unwrap()
    );
    assert_eq!(
        deserialized["fieldC"],
        Value::from(&[1u8, 2, 3][..]).try_into().unwrap()
    );

    let serialized = zvariant::to_bytes(ctxt, &deserialized).unwrap();
    let deserialized: Test = serialized.deserialize().unwrap().0;

    assert_eq!(deserialized.field_a, Some(1u32));
    assert_eq!(deserialized.field_b.as_str(), "foo");
    assert_eq!(deserialized.field_c.as_slice(), &[1u8, 2, 3][..]);

    assert_eq!(Test::SIGNATURE, "a{sv}")
}

#[test]
#[ignore]
fn issues_311() {
    // Issue 311: Value macro not able to handle Option in Dict.
    //
    // org.freedesktop.ModemManager1.Modem.Signal props are a dict with optional values depending on
    // the property you read.
    #[derive(Debug, Type, DeserializeDict, OwnedValue, Value)]
    #[zbus(signature = "dict")]
    pub struct SignalInfo {
        pub rssi: Option<i32>,
        pub ecio: Option<i32>,
        pub io: Option<i32>,
        pub sinr: Option<i32>,
    }
}

#[test]
#[ignore]
fn issues_1252() {
    // Issue 1252: Naming a field `key` in a dict struct causes a conflict with variables created by
    // `DeserializeDict` macro, ending up with a strange error.
    #[derive(DeserializeDict, Type)]
    #[zvariant(signature = "a{sv}")]
    pub struct OwnedProperties {
        key: String,
        val: OwnedValue,
    }
}
