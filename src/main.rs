use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct AppRequest {
	receive_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Secret {
	username: String,
	password: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
	let mut app = tide::new();
	app.at("/sendSecretToMe").post(order_shoes);
	app.listen("0.0.0.0:8000").await?;
	Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
	let AppRequest { receive_url } = req.body_json().await?;
	let y: u8 = rand::random();
	if y > 210u8 {
		let mut receive_url_2 = String::new();
		receive_url_2.push_str(&receive_url);
		async_std::task::spawn(async {
			let data = Secret {
				username: "user_a".to_owned(),
				password: "password_a".to_owned(),
			};
			surf::post(receive_url_2)
				.body_json(&data)
				.unwrap()
				.await
				.unwrap()
		});
		return Ok(format!(
			"Hello {:?} ! I've send the secrets to {}\n",
			req.peer_addr(),
			receive_url
		)
		.into());
	} else {
		return Ok(format!(
			"Hello {:?} ! I've send the secrets to  {}\n",
			req.peer_addr(),
			receive_url
		)
		.into());
	}
}
