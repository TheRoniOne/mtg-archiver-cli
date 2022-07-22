mod retrieve;
fn main() {
    let body = retrieve::retrieve_bulk_data_info().unwrap().text().unwrap();
    println!("{:#?}", body)
}
