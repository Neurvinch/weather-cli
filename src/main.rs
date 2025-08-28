fn main() {
    let api_token = std::env::var("API_TOKEN")
    .expect("expected there to be an api token");
}



  enum Result<THING_WE_WANT,ERROR_THATCOULD_HAPPEN>{
    Ok(THING_WE_WANT),
    Err(ERROR_THATCOULD_HAPPEN)
  }

