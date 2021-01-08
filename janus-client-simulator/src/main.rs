use reqwest::Url;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct JanusRequest {
    janus: String,
    transaction: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JanusResponse<T> {
    janus: String,
    transaction: String,
    data: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct Session {
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttachPlugin {
    janus: String,
    plugin: String,
    transaction: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PluginHandle {
    id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct WatchRequest {
    janus: String,
    transaction: String,
    body: WatchRequestBody,
}

#[derive(Serialize, Deserialize, Debug)]
struct WatchRequestBody {
    request: String,
    channelId: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let janus_url = Url::parse("https://server.stream.danielstiner.me/janus")?;
    let channel_id = 1;

    let client = reqwest::blocking::Client::new();

    let request = JanusRequest {
        janus: String::from("create"),
        transaction: Uuid::new_v4().to_string(),
    };
    let response = client.post(janus_url.as_ref()).json(&request).send()?;
    println!("{:#?}", response);
    let response_json = response.json::<JanusResponse<Session>>()?;
    println!("{:#?}", response_json);
    assert_eq!("success", &response_json.janus);
    assert_eq!(&request.transaction, &response_json.transaction);
    let session_id = response_json.data.id;

    let session_url = janus_url.join(&format!("janus/{}", session_id))?;

    // let response = client.get(session_url.as_ref()).send()?;
    // println!("{:#?}", response);
    // println!("{:#?}", response.text()?);

    let request = AttachPlugin {
        janus: String::from("attach"),
        plugin: String::from("janus.plugin.ftl"),
        transaction: Uuid::new_v4().to_string(),
    };
    let response = client.post(session_url.as_ref()).json(&request).send()?;
    println!("{:#?}", response);
    let response_json = response.json::<JanusResponse<PluginHandle>>()?;
    println!("{:#?}", response_json);
    assert_eq!("success", &response_json.janus);
    assert_eq!(&request.transaction, &response_json.transaction);
    let plugin_handle = response_json.data.id;

    let plugin_url = janus_url.join(&format!("janus/{}/{}", session_id, plugin_handle))?;

    let request = WatchRequest {
        janus: String::from("message"),
        transaction: Uuid::new_v4().to_string(),
        body: WatchRequestBody {
            request: String::from("watch"),
            channelId: channel_id,
        }
    };
    let response = client.post(plugin_url.as_ref()).json(&request).send()?;
    println!("{:#?}", response);
    let response_json = response.json::<JanusResponse<()>>()?;
    println!("{:#?}", response_json);
    assert_eq!("success", &response_json.janus);
    assert_eq!(&request.transaction, &response_json.transaction);

    Ok(())
}
