use reqwest::Client;

// use weather::location;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = "https://api.weather.gov";

    let latitude: f64 = 42.440932990492946;
    let longitude: f64 = -76.52462385924595;
    let coords = format!("{},{}", round_fmt(latitude, 4), round_fmt(longitude, 4));
    let url = format!("{}/points/{}", api, coords);

    let client = Client::builder()
        .user_agent("(weather.allotropic.com, zach@allotropic.com)")
        .build()?;

    println!("{}", url);
    let response = client.get(url).send().await?;
    println!("{:#?}", response.text().await?);

    Ok(())
}

fn round_fmt(f: f64, digits: u32) -> String {
    let pow = 10u32.pow(digits) as f64;
    let f = (f * pow).round() / pow;

    format!("{0:.1$}", f, digits as usize)
}

#[cfg(test)]
mod tests {
    use super::round_fmt;

    #[test]
    fn round_coords() {
        #![allow(clippy::excessive_precision)]
        let latitude: f64 = 53.473894723894738;
        let longitude: f64 = -39.43784723847389;

        assert_eq!("53.4739".to_string(), round_fmt(latitude, 4));
        assert_eq!("-39.4378".to_string(), round_fmt(longitude, 4));
    }
}
