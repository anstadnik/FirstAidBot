use model::get_data;

mod model;

fn main() {
    // Prod
    // let sheet_id = "1cO0sPRhIvt71J-iB313BeRfNXzXM0FjiQ4bDYmwddBQ";
    // Test
    let sheet_id = "1seobblWaZXSu82yf3CnanIps26vCv3QARo75-sAC2KQ";
    let sheet_name = "Sheet1";
    let data = get_data(sheet_id, sheet_name);
    println!("{data:#?}");
}
