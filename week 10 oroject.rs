struct Laptop {
    brand: String,
    price_per_unit: u32,
    available_quantity: u32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u32) -> u32 {
        if quantity <= self.available_quantity {
            self.price_per_unit * quantity
        } else {
            panic!(
                "Not enough {} laptops in stock. Available: {}, Requested: {}",
                self.brand, self.available_quantity, quantity
            );
        }
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price_per_unit: 650_000,
        available_quantity: 10,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price_per_unit: 755_000,
        available_quantity: 6,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price_per_unit: 550_000,
        available_quantity: 10,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price_per_unit: 850_000,
        available_quantity: 4,
    };

    let total_cost = hp.calculate_cost(3)
        + ibm.calculate_cost(3)
        + toshiba.calculate_cost(3)
        + dell.calculate_cost(3);

    println!("The total cost for purchasing 3 laptops from each brand is: ₦{}", total_cost);
}

