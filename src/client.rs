use reqwest;

async fn send_request() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:3300/hello/worldd").await?;

    println!("{:#?}", resp);

    Ok(())
}

pub async fn run_client() {
    tokio::spawn(async {
        loop {
            let mut text = String::new();

            std::io::stdin().read_line(&mut text).unwrap();

            let text = text.trim();

            match text {
                "send" => {
                    println!("sending");
                    send_request().await.unwrap();
                }

                "kill" => {
                    println!("killing");
                    break;
                }

                _ => println!("{text}"),
            }
        }
    })
    .await
    .unwrap();
}
