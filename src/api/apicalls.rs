use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

const URL: &str = "https://backend-rust-277582128315.us-central1.run.app/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trip {
    pub vendor_id: String,
    pub tpep_pickup_datetime: String,
    pub tpep_dropoff_datetime: String,
    pub passenger_count: String,
    pub trip_distance: String,
    pub ratecode_id: String,
    pub store_and_fwd_flag: String,
    pub pu_location_id: String,
    pub do_location_id: String,
    pub payment_type: String,
    pub fare_amount: String,
    pub extra: String,
    pub mta_tax: String,
    pub tip_amount: String,
    pub tolls_amount: String,
    pub improvement_surcharge: String,
    pub total_amount: String,
    pub congestion_surcharge: String,
    pub index: String,
}

pub async fn is_alive() -> Result<bool, Box<dyn Error>> {
    reqwest::get(URL).await?.text().await?;
    println!("Server is alive");
    Ok(true)
}

pub async fn get_by_index(index: String) -> Result<Trip, Box<dyn Error>> {
    let url: String = URL.to_owned() + "/trip/" + &index;
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let trip: Trip = response.json().await?;
        Ok(trip)
    } else {
        Err(format!("Error: Status {}", response.status()).into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetByPriceRangeInput {
    pub min: String,
    pub max: String,
    pub pages: String,
    pub per_page: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetByPriceRangeOutput {
    pub items: Vec<Trip>,
    pub page: u32,
    pub pages: u32,
    pub per_page: u32,
    pub time_ms: u32,
    pub total: u32,
}

pub async fn get_by_price_range(
    data: &GetByPriceRangeInput,
) -> Result<GetByPriceRangeOutput, Box<dyn Error>> {
    let client = Client::new();
    let url: String = URL.to_owned() + "/trip/price";
    let response = client.get(url).query(data).send().await?;

    if response.status().is_success() {
        let output: GetByPriceRangeOutput = response.json().await?;
        Ok(output)
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Error desconocido".to_string());
        Err(format!("Error {}: {}", status, error_text).into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetByDestinationInput {
    pub destination: String,
    pub pages: String,
    pub per_page: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetByDestinationQuery {
    pub pages: String,
    pub per_page: String,
}

async fn get_by_destination(
    data: &GetByDestinationInput,
) -> Result<GetByPriceRangeOutput, Box<dyn Error>> {
    let client = Client::new();
    let query: GetByDestinationQuery = GetByDestinationQuery {
        pages: data.pages.clone(),
        per_page: data.per_page.clone(),
    };
    let url: String = URL.to_owned() + "/trip/destination/" + &data.destination;
    let response = client.get(url).query(&query).send().await?;
    if response.status().is_success() {
        let output: GetByPriceRangeOutput = response.json().await?;
        Ok(output)
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Error desconocido".to_string());
        Err(format!("Error {}: {}", status, error_text).into())
    }
}
