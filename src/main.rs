use prost::Message;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use chrono::Utc;

mod customer {
    include!(concat!(env!("OUT_DIR"), "/customer.rs"));
}

use customer::Customer;

fn serialize_customer(customer: &Customer) -> Vec<u8> {
    let mut buf = Vec::new();
    customer.encode(&mut buf).unwrap();
    buf
}

fn calculate_checksum(customer: &Customer) -> u32 {
    let mut modified_customer = customer.clone();
    modified_customer.header.as_mut().unwrap().id = 0;
    modified_customer.header.as_mut().unwrap().timestamp = 0;

    let serialized_customer = serialize_customer(&modified_customer);

    serialized_customer.iter().map(|&byte| byte as u32).sum()
}

fn create_customer() -> Customer {
    let mut customer = Customer {
        header: Some(customer::Header {
            id: 1,
            timestamp: Utc::now().timestamp() as u32,
        }),
        name: "John Doe".to_string(),
        contact_info: Some(customer::customer::ContactInfo::Email("john.doe@example.com".to_string())),
        // contact_info: Some(customer::customer::ContactInfo::Phone("123456".to_string())),
        address: "123 Main St, Anytown, USA".to_string(),
        footer: Some(customer::Footer {
            checksum: 0,
        }),
    };
    customer.footer.as_mut().unwrap().checksum = calculate_checksum(&customer);
    customer
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
    println!("---Header---");
    println!("Customer ID: {}", deserialized_customer.header.unwrap().id);
    println!("Timestamp: {}", deserialized_customer.header.unwrap().timestamp);
    println!("---Payload---");
    println!("Name: {}", deserialized_customer.name);
    match &deserialized_customer.contact_info {
        Some(customer::customer::ContactInfo::Email(email)) => {
            println!("Email: {}", email);
        }
        Some(customer::customer::ContactInfo::Phone(phone)) => {
            println!("Phone: {}", phone);
        }
        None => {
            println!("No contact info provided.");
        }
    }
    println!("Address: {}", deserialized_customer.address);
    println!("---Footer---");
    println!("Checksum: {}", deserialized_customer.footer.unwrap().checksum);
}