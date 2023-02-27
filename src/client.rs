use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".into(),
        to_addr: "123123123".into(),
        amount: 1,
    });

    let response = client.send_payment(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
