use crate::*;

#[test]
fn it_works() {
    let json = r#"[
          {
            "name": "sksat",
            "uuid": "eab5e33d-8305-4e3d-aa03-8cf527faac7b"
          }
        ]"#;

    let _whitelist: minecraft::UserList = serde_json::from_str(json).unwrap();
}

#[tokio::test]
async fn mc_yohane_su() -> Result<(), reqwest::Error> {
    let url = "https://raw.githubusercontent.com/sksat/mc.yohane.su/main/data/whitelist.json";

    let json = reqwest::get(url).await?.text().await?;
    println!("whitelist.json: {}", json);

    let whitelist: minecraft::UserList = serde_json::from_str(&json).unwrap();

    for user in whitelist {
        if user.exist().await.unwrap() {
            continue;
        }

        panic!("\"{}\" does not exist!", user.name);
    }

    Ok(())
}
