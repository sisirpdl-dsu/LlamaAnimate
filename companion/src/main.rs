use serde_json::json;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

use std::process::Command;
use serde_json::Value;

fn create_file(message: String, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    let _ = file.write(message.as_bytes())?;
    Ok(())
}
fn create_wav() -> io::Result<()> {
    let _ = Command::new("bash")
        .args(["script.sh"])
        .output()
        .expect("script failed");
    //let _ = Command::new("bash").args(["-c","touch hello"]).spawn().expect("script failed");
    Ok(())
}
use std::io;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    //let x = reqwest::get("http://localhost:11434/").await?;

    let mut input = String::new();
let model = "llama3.2";
let companion_name = "Emily";
let msg = json!(
        {"model": model,
        });
            client
            .post("http://localhost:11434/api/pull")
            .header("Content-Type", "application/json")
            .body(msg.to_string())
            .send()
            .await?;
    let mut chat: Vec<Value> = Vec::new();
    chat.push(json!(
            {"role":"system","content":format!("You are my girlfriend. Your name is {companion_name} Please do short responses. ")})
    );


    loop {
        loop {
            let z = io::stdin().read_line(&mut input);
            if z.is_ok() {
                break;
            }
            println!();
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("bye") {
            break;
        }
        chat.push(json!(
        {
            "role":"user","content":format!("{input}")
        }
        ));


        let msg = json!(
            {"model": "llama3.2",

            //"messages":[{"role":"system","content":&format!("You are my girlfriend. Your name is {companion_name} Please do short responses. ")},{"role":"user","content":format!("{input}")}],
            "messages":Value::Array(chat.clone()),

            "options":{
                "repeat_penalty": 1.2,
            }

            });

        //msg["messages"]= Value::Array(chat.clone());

        //println!("{}",msg.get("messages").unwrap());
        let mut reply = String::new();
        if let Ok(response) = client
            .post("http://localhost:11434/api/chat")
                .header("Content-Type", "application/json")
                .body(msg.to_string())
                .send()
                .await
        {
            //let r = response.json::<serde_json::Value>().await?;
            let body = response.text().await?;
            let body: Vec<&str> = body.split('\n').collect();
            for token in body {
                let json: Result<HashMap<String, serde_json::Value>, serde_json::Error> =
                    serde_json::from_str(token);
                if json.is_ok() {
                    let json = json.unwrap();
                    let temp = json.get("message").unwrap();

                    chat.push(temp.clone());

                    let temp = temp.get("content").unwrap();
                    let temp = temp.to_string();
                    let temp = temp.trim();
                    let temp = temp.replace("\\n", "");
                    let temp = temp.replace("\\", "");
                    let part = &temp[1..(temp.len() - 1)]; //llama3 or deepseek

                    reply+=part;
                    //reply += &temp[0..(temp.len() - 0)]; //other
                    //reply = temp[1..(temp.len() - 1)].to_string();
                    //print!(" {reply} ");
                    //reply += &temp[0..];
                }
            }
        }
        let filename = "reply.txt".to_string();
        let _ = create_file(reply.clone(), &filename);
        let r = create_wav();
        println!("{companion_name}: {}", reply);
        match r {
            Ok(_) => {}
            Err(e) => {
                dbg!("{}", e.to_string());
            }
        }
    }
    Ok(())
}
