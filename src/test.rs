use crate::*;

#[test]
fn it_works() {
    let json = r#"[
          {
            "name": "sksat",
            "uuid": "eab5e33d-8305-4e3d-aa03-8cf527faac7b"
          }
        ]"#;

    let _whitelist: UserList = serde_json::from_str(json).unwrap();
}

#[test]
fn mc_yohane_su() {
    let json = include_str!("../test/yohanesu_whitelist.json");

    let _whitelist: UserList = serde_json::from_str(json).unwrap();
}
