use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

use glob::glob;
use serde::{Deserialize, Serialize};
use tokio::{process::Command, sync::Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Config {
    sign_tool: String,
    args: Vec<String>,
    include: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Detail {
    code: i32,
    msg: String,
}

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from("./signature.config.json");
    let config_string = fs::read_to_string(config_path)
        .expect("❌ Configuration file signature.config.json cannot be read");
    let config = serde_json::from_str::<Config>(&config_string)
        .expect("❌ The configuration file format is incorrect");

    println!("config: {config:?}");

    let config_sign_tool = Arc::new(config.sign_tool);
    let config_args = Arc::new(config.args);
    let result: Arc<Mutex<HashMap<String, Detail>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for pattern in config.include {
        let mut matched = false;
        for entry in glob(&pattern).unwrap() {
            matched = true;

            let file_path = match entry {
                Ok(v) => v,
                Err(_) => continue,
            };

            let sign_tool_clone = Arc::clone(&config_sign_tool);
            let config_args_clone = Arc::clone(&config_args);
            let result_clone = Arc::clone(&result);

            let task = tokio::spawn(async move {
                println!("signature {file_path:?}");

                let output = match Command::new(&*sign_tool_clone)
                    .args(config_args_clone.as_ref())
                    .arg(&file_path)
                    .output()
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        result_clone.lock().await.insert(
                            file_path.to_string_lossy().to_string(),
                            Detail {
                                code: -1,
                                msg: e.to_string(),
                            },
                        );

                        println!("❌ signature {file_path:?} - Error: {e:?}");

                        return;
                    }
                };

                let detail = if output.status.success() {
                    Detail {
                        code: 0,
                        msg: String::from_utf8_lossy(&output.stdout)
                            .replace("\r\n", "\n")
                            .trim_end()
                            .to_string(),
                    }
                } else {
                    Detail {
                        code: output.status.code().unwrap_or(-1),
                        msg: String::from_utf8_lossy(&output.stderr)
                            .replace("\r\n", "\n")
                            .trim_end()
                            .to_string(),
                    }
                };

                result_clone
                    .lock()
                    .await
                    .insert(file_path.to_string_lossy().to_string(), detail);
            });

            tasks.push(task);
        }

        if !matched {
            eprintln!("⚠️ No files matched pattern: {pattern}");
        }
    }

    for task in tasks {
        let _ = task.await;
    }

    println!("result: {:?}", result.lock().await);

    let result_json =
        serde_json::to_string_pretty(&*result.lock().await).unwrap_or_else(|_| "err".to_string());
    fs::write("result.json", result_json).unwrap();
}
