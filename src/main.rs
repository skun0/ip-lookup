use serde::Deserialize;
use std::io;
use reqwest;
use tokio;

#[derive(Deserialize, Debug)]
struct IpInfo {
    query: Option<String>,
    status: String,
    country: Option<String>,
    #[serde(rename = "regionName")]
    region_name: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    isp: Option<String>,
    message: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B]0;https://github.com/skun0\x07");
    let mut ip = String::new();
    println!("IP: \n");
    io::stdin().read_line(&mut ip)?;
    let ip = ip.trim();

    let url = if ip.is_empty() {
        "http://ip-api.com/json/".to_string()
    } else {
        format!("http://ip-api.com/json/{}", ip)
    };

    let r = reqwest::get(&url)
        .await?
        .json::<IpInfo>()
        .await?;

    if r.status == "success" {
        print!("\x1B[2J\x1B[1;1H");
        println!("IP: {}", r.query.unwrap_or_else(|| "N/A".to_string()));
        println!("Country: {}", r.country.unwrap_or_else(|| "N/A".to_string()));
        println!("Region: {}", r.region_name.unwrap_or_else(|| "N/A".to_string()));
        println!("City: {}", r.city.unwrap_or_else(|| "N/A".to_string()));
        println!("ZIP: {}", r.zip.unwrap_or_else(|| "N/A".to_string()));
        println!("Latitude: {}", r.lat.unwrap_or(0.0));
        println!("Longitude: {}", r.lon.unwrap_or(0.0));
        println!("ISP: {}", r.isp.unwrap_or_else(|| "N/A".to_string()));
    } else {
        println!("Failed: {}", r.message.unwrap_or_else(|| "Unknown error".to_string()));
    }
    let mut exit = String::new();
    println!("");
    io::stdin().read_line(&mut exit)?;
    Ok(())
}