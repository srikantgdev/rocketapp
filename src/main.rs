#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rusqlite::Connection;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
// use rocket::fairing::{Fairing, Info, Kind};
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct CategoryList {
    data: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    category_id: i64,
    category_name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct ProductList {
    data: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    product_id: i64,
    product_name: String,
    category: String,
    qty_per_unit: String,
    unit_price: String,
    reorder_level: i32
}

#[derive(Serialize, Deserialize)]
struct CustomerList {
    data: Vec<Customer>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Customer {
    customer_id: String,
    company_name: String,
    contact_title: String,
    contact_name: String,
    city: String,
    country: String,
}

#[derive(Serialize)]
struct StatusMessage{
    message: String,
}

#[derive(Serialize)]
struct Temperature {
    cel: i32,
    farh: i32
}

#[derive(Serialize)]
struct App {
    title: String
}

#[get("/")]
// fn index() -> &'static str {
//     "'/': Home | '/categories': Categories | '/products': Products | '/cel/10/farh': Cel to Farh | '/farh/20/cel': Farh to Cel"
// }
fn index() -> Template {
    Template::render("index", App {title: "App Routes".to_owned()})
}

#[get("/categories")]
fn getcategories() -> Result<Template, String> {
    let db_connection = match Connection::open("northwind.db") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };
    
    let mut statement = match db_connection.prepare("select CategoryID, CategoryName, Description from Categories") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    
    
    let results = statement.query_map([], |row| {
        Ok(Category {
            category_id: row.get(0)?,
            category_name: row.get(1)?,
            description: row.get(2)?,
        })
    });
    
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<Category>> = rows.collect();
            
            match collection {
                Ok(data) => Ok(Template::render("categories", CategoryList {data})),
                Err(err) => Err(String::from(err.to_string()))
            }
        }
        Err(_) => Err(String::from("Error while fetching categories"))
    }
}

/**
 * products for category
 */
#[get("/products/<catid>")]
fn getproductsforcategory(catid: String) -> Result<Template, String> {
    let db_connection = match Connection::open("northwind.db") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let mut statement = match db_connection.prepare(&format!("Select ProductID, ProductName, QuantityPerUnit, UnitPrice, ReorderLevel from Products Where CategoryID={}",catid)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    
    let results = statement.query_map([], |row| {
        Ok(Product {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            category: row.get(2)?,
            qty_per_unit: row.get(3)?,
            unit_price: row.get(4)?,
            reorder_level: row.get(5)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<Product>> = rows.collect();

            match collection {
                Ok(data) => Ok(Template::render("products", ProductList { data })),
                Err(err) => Err(String::from(err.to_string())),
            }
        }
        Err(_) => Err(String::from("Error while fetching products"))
    }
}

#[get("/products")]
fn getproducts() -> Result<Template, String> {
    let db_connection = match Connection::open("northwind.db") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };

    let mut statement = match db_connection.prepare("select p.ProductID, p.ProductName, c.CategoryName as category, p.QuantityPerUnit, printf('%.2f',p.UnitPrice) as UnitPrice, ReorderLevel from Products p Join Categories c on c.CategoryID=p.CategoryID") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    
    let results = statement.query_map([], |row| {
        Ok(Product {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            category: row.get(2)?,
            qty_per_unit: row.get(3)?,
            unit_price: row.get(4)?,
            reorder_level: row.get(5)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<Product>> = rows.collect();

            match collection {
                Ok(data) => Ok(Template::render("products", ProductList { data })),
                Err(err) => Err(String::from(err.to_string())),
            }
        }
        Err(_) => Err(String::from("Error while fetching products"))
    }
}

#[get("/customers")]
fn getcustomers() -> Result<Template, String> {
    let db_connection = match Connection::open("northwind.db") {
        Ok(connection) => connection,
        Err(err) => return Err(String::from(err.to_string())),
    };
    
    let mut statement = match db_connection.prepare("select CustomerID, CompanyName, ContactTitle, ContactName, IfNull(City,'') as City, IfNull(Country,'') as Country From Customers") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    
    
    let results = statement.query_map([], |row| {
        Ok(Customer {
            customer_id: row.get(0)?,
            company_name: row.get(1)?,
            contact_title: row.get(2)?,
            contact_name: row.get(3)?,
            city: row.get(4)?,
            country: row.get(5)?,
        })
    });
    
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<Customer>> = rows.collect();
            
            match collection {
                Ok(data) => Ok(Template::render("customers", CustomerList { data })),
                Err(err) => Err(String::from(err.to_string()))
            }
        }
        Err(_) => Err(String::from("Error while fetching customers"))
    }
}

#[get("/cel/<cel>/farh")]
fn ctof(cel: i32) -> Result<Json<Temperature>, String> {
    let f = (cel * 9 / 5) + 32;
    let temp = Temperature{cel: cel, farh: f};
    Ok(Json(temp))
}
#[get("/farh/<f>/cel")]
fn ftoc(f: i32) -> Result<Json<Temperature>, String> {
    let cel = (f - 32) * 5 / 9;
    let temp = Temperature{cel: cel, farh: f};
    Ok(Json(temp))
}

fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::all();
    
    let cors = CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin"
        ]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()?;
    
    rocket::ignite()
    .attach(Template::fairing())
    .attach(cors)
    .mount("/", routes![
        index,
        getcategories,
        getproductsforcategory,
        getproducts,
        getcustomers,
        ctof,
        ftoc
    ])
    .launch();
    Ok(())
}
