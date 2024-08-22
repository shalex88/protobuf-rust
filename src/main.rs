use prost::Message;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

mod customer {
    include!(concat!(env!("OUT_DIR"), "/customer.rs"));
}

use customer::Customer;

fn create_customer() -> Customer {
    Customer {
        id: 1,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        address: "123 Main St, Anytown, USA".to_string(),
    }
}

fn serialize_customer(customer: &Customer) -> Vec<u8> {
    let mut buf = Vec::new();
    customer.encode(&mut buf).unwrap();
    buf
}

fn write_serialized_customer_to_file(data: &Vec<u8>, filename: &str) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&data).unwrap();
}

fn read_customer_from_file(filename: &str) -> Customer {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    Customer::decode(&*buf).unwrap()
}

fn main() {
    println!("protobuf-rust");

    // Create a customer
    let customer = create_customer();

    // Serialize customer
    let serialized_customer = serialize_customer(&customer);

    // Write serialized customer to file
    write_serialized_customer_to_file(&serialized_customer, "customer.dat");

    println!("Customer data serialized and written to file.");

    // Read and deserialize the customer
    let deserialized_customer = read_customer_from_file("customer.dat");

    // Display the customer's information
    println!("Customer ID: {}", deserialized_customer.id);
    println!("Name: {}", deserialized_customer.name);
    println!("Email: {}", deserialized_customer.email);
    println!("Address: {}", deserialized_customer.address);
}