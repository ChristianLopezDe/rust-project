//For HTML
use rocket_dyn_templates::context;
//For CSS
use rocket::fs::{FileServer, relative};
//For Get Request and Resposne
use rocket::serde::{Serialize, Deserialize}; //Convert data to JSON format
//#[feature(proc_macro_hygiene, decl_macro)]  For secret key
//use rocket::{fs::NamedFile, response::Redirect}; maybe no
//use std::path::{Path, PathBuf};
#[macro_use] extern crate rocket;
use rocket::{State, form::*, get, post, response::Redirect, routes};
use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use sqlx::*;
use rocket::serde::json::json;

use std::result::Result;
use std::*;

//Ideally inventory is grabbed from the values in sqlite database. Not sure how to do that however.

static mut website_inv: Inventory = { Inventory {
    apple_stock: 50,
    banana_stock: 150,
    broccoli_stock: 200,
    watermelon_stock: 5,
    orange_stock:225,
    grape_stock: 75,
} } ;

static mut u_stock: UserStock = { UserStock{
    apple_count: 0,
    banana_count: 0,
    broccoli_count: 0,
    watermelon_count: 0,
    orange_count: 0,
    grape_count: 0,
}
};

#[derive(Serialize, Deserialize, FromForm)]
struct Inventory {
    apple_stock: u32,
    banana_stock: u32,
    broccoli_stock: u32,
    watermelon_stock: u32,
    orange_stock: u32,
    grape_stock: u32,
}

#[derive(Serialize, Deserialize, FromForm)]
struct UserStock{
    apple_count: u32,
    banana_count: u32,
    broccoli_count: u32,
    watermelon_count: u32,
    orange_count: u32,
    grape_count: u32
}

//The webpages...
#[get("/db")]
async fn get_login(user: Option<User>) -> Template {
    unsafe {
        Template::render("Dashboard", json!({ "user": user, "website_inv": website_inv, "u_stock":u_stock }))
    }
}

#[post("/login", data = "<form>")]
async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    println!("login attempt: {:?}", result);
    result?;
    Ok(Redirect::to("/db"))
}

#[post("/signup", data = "<form>")]
async fn post_signup(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok(Redirect::to("/signup"))
}

#[get("/signup")]
async fn sign_up_page(user: Option<User>) -> Template {
    Template::render("NewUser", json!({ "user": user }))
}

#[get("/logout")]
fn logout(auth: Auth<'_>) ->  Result<Redirect, Error> {
    auth.logout()?;
    Ok(Redirect::to("/"))
}

#[get("/update")]
async fn update(user: Option<User>) -> Template {
    unsafe {
        Template::render("UpdatePage", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[get("/")]
fn index() -> Template {
    Template::render ("HomePage", context! { })
}

#[derive (Debug,FromForm)]
struct Submission<> {
    apple_count : u32,
    banana_count : u32,
    broccoli_count : u32,
    watermelon_count : u32,
    orange_count : u32,
    grape_count : u32,
}

#[post ("/newUserSubmit", data = "<s>")]
fn formsub(s : Form<Submission<>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        /*Update the users stock. Preferably done in backend. */
        u_stock.apple_count = s.apple_count;
        u_stock.banana_count = s.banana_count;
        u_stock.broccoli_count = s.broccoli_count;
        u_stock.watermelon_count = s.watermelon_count;
        u_stock.orange_count = s.orange_count;
        u_stock.grape_count = s.grape_count;

        Ok(Redirect::to("/db"))
    }
}

#[get("/tradesuccess")]
async fn after_trade(user: Option<User>) -> Template {
    unsafe {
        Template::render("Trades/TradeSuccess", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[derive (Debug,FromForm)]
struct AppleForm<'r> {
    apple_count: u32,
    fruit: &'r str,
}

#[post ("/AppleTrade", data = "<s>")]
fn appleform(s : Form<AppleForm<'_>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        let fruit = s.fruit;
        let count = s.apple_count;
        //1. Update the users stock. ADD
        u_stock.apple_count = u_stock.apple_count + count;
        //2. Update user stock. Subtract traded fruit from user stock. At same time, add to website inventory.
        if fruit == "Banana" {
            u_stock.banana_count =  u_stock.banana_count - count;
            website_inv.banana_stock =  website_inv.banana_stock + count;
        } else if fruit == "Watermelon"{
            u_stock.watermelon_count =  u_stock.watermelon_count - count;
            website_inv.watermelon_stock =  website_inv.watermelon_stock + count;
        } else if fruit == "Orange"{
            u_stock.orange_count =  u_stock.orange_count - count;
            website_inv.orange_stock =  website_inv.orange_stock + count;
        } else if fruit == "Broccoli"{
            u_stock.broccoli_count =  u_stock.broccoli_count - count;
            website_inv.broccoli_stock =  website_inv.broccoli_stock + count;
        } else if fruit == "Grape"{
            u_stock.grape_count =  u_stock.grape_count - count;
            website_inv.grape_stock =  website_inv.grape_stock + count;
        }
        //3. Update website stock. Subtract traded fruit
        website_inv.apple_stock = website_inv.apple_stock - count; 

        Ok(Redirect::to("tradesuccess"))
    }
}

#[get("/AppleTrade")]
async fn apple(user: Option<User>) -> Template {
    unsafe{
        Template::render("FoodPages/ApplePage", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[derive (Debug,FromForm)]
struct BananaForm<'r> {
    banana_count: u32,
    fruit: &'r str,
}

#[post ("/BananaTrade", data = "<s>")]
fn bananaform(s : Form<BananaForm<'_>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        let fruit = s.fruit;
        let count = s.banana_count;
        //1. Update the users stock. ADD
        u_stock.banana_count = u_stock.banana_count + count;
        //2. Update user stock. Subtract traded fruit from user stock. At same time, add to website inventory.
        if fruit == "Apple" {
            u_stock.apple_count =  u_stock.apple_count - count;
            website_inv.apple_stock =  website_inv.apple_stock + count;
        } else if fruit == "Watermelon"{
            u_stock.watermelon_count =  u_stock.watermelon_count - count;
            website_inv.watermelon_stock =  website_inv.watermelon_stock + count;
        } else if fruit == "Orange"{
            u_stock.orange_count =  u_stock.orange_count - count;
            website_inv.orange_stock =  website_inv.orange_stock + count;
        } else if fruit == "Broccoli"{
            u_stock.broccoli_count =  u_stock.broccoli_count - count;
            website_inv.broccoli_stock =  website_inv.broccoli_stock + count;
        } else if fruit == "Grape"{
            u_stock.grape_count =  u_stock.grape_count - count;
            website_inv.grape_stock =  website_inv.grape_stock + count;
        }
        //3. Update website stock. Subtract traded fruit
        website_inv.banana_stock = website_inv.banana_stock - count; 

        Ok(Redirect::to("/tradesuccess"))
    }
}

#[get("/BananaTrade")]
async fn banana(user: Option<User>) -> Template {
    unsafe{
        Template::render("FoodPages/BananaPage", json!({ "user": user , "u_stock":u_stock}))
    }
}

#[derive (Debug,FromForm)]
struct OrangeForm<'r> {
    orange_count: u32,
    fruit: &'r str,
}

#[post ("/OrangeTrade", data = "<s>")]
fn orangeform(s : Form<OrangeForm<'_>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        let fruit = s.fruit;
        let count = s.orange_count;
        //1. Update the users stock. ADD
        u_stock.orange_count = u_stock.orange_count + count;
        //2. Update user stock. Subtract traded fruit from user stock. At same time, add to website inventory.
        if fruit == "Apple" {
            u_stock.apple_count =  u_stock.apple_count - count;
            website_inv.apple_stock =  website_inv.apple_stock + count;
        } else if fruit == "Watermelon"{
            u_stock.watermelon_count =  u_stock.watermelon_count - count;
            website_inv.watermelon_stock =  website_inv.watermelon_stock + count;
        } else if fruit == "Banana"{
            u_stock.banana_count =  u_stock.banana_count - count;
            website_inv.banana_stock =  website_inv.banana_stock + count;
        } else if fruit == "Broccoli"{
            u_stock.broccoli_count =  u_stock.broccoli_count - count;
            website_inv.broccoli_stock =  website_inv.broccoli_stock + count;
        } else if fruit == "Grape"{
            u_stock.grape_count =  u_stock.grape_count - count;
            website_inv.grape_stock =  website_inv.grape_stock + count;
        }
        //3. Update website stock. Subtract traded fruit
        website_inv.orange_stock = website_inv.orange_stock - count; 

        Ok(Redirect::to("/tradesuccess"))
    }
}


#[get("/OrangeTrade")]
async fn orange(user: Option<User>) -> Template {
    unsafe{
        Template::render("FoodPages/OrangePage", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[derive (Debug,FromForm)]
struct WatermelonForm<'r> {
    watermelon_count: u32,
    fruit: &'r str,
}

#[post ("/WatermelonTrade", data = "<s>")]
fn watermelonform(s : Form<WatermelonForm<'_>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        let fruit = s.fruit;
        let count = s.watermelon_count;
        //1. Update the users stock. ADD
        u_stock.watermelon_count = u_stock.watermelon_count + count;
        //2. Update user stock. Subtract traded fruit from user stock. At same time, add to website inventory.
        if fruit == "Apple" {
            u_stock.apple_count =  u_stock.apple_count - count;
            website_inv.apple_stock =  website_inv.apple_stock + count;
        } else if fruit == "Orange"{
            u_stock.orange_count =  u_stock.orange_count - count;
            website_inv.orange_stock =  website_inv.orange_stock + count;
        } else if fruit == "Banana"{
            u_stock.banana_count =  u_stock.banana_count - count;
            website_inv.banana_stock =  website_inv.banana_stock + count;
        } else if fruit == "Broccoli"{
            u_stock.broccoli_count =  u_stock.broccoli_count - count;
            website_inv.broccoli_stock =  website_inv.broccoli_stock + count;
        } else if fruit == "Grape"{
            u_stock.grape_count =  u_stock.grape_count - count;
            website_inv.grape_stock =  website_inv.grape_stock + count;
        }
        //3. Update website stock. Subtract traded fruit
        website_inv.watermelon_stock = website_inv.watermelon_stock - count; 

        Ok(Redirect::to("/tradesuccess"))
    }
}

#[get("/WatermelonTrade")]
async fn watermelon(user: Option<User>) -> Template {
    unsafe{
        Template::render("FoodPages/WatermelonPage", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[derive (Debug,FromForm)]
struct BroccoliForm<'r> {
    broccoli_count: u32,
    fruit: &'r str,
}

#[post ("/BroccoliTrade", data = "<s>")]
fn broccoliform(s : Form<BroccoliForm<'_>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        let fruit = s.fruit;
        let count = s.broccoli_count;
        //1. Update the users stock. ADD
        u_stock.broccoli_count = u_stock.broccoli_count + count;
        //2. Update user stock. Subtract traded fruit from user stock. At same time, add to website inventory.
        if fruit == "Apple" {
            u_stock.apple_count =  u_stock.apple_count - count;
            website_inv.apple_stock =  website_inv.apple_stock + count;
        } else if fruit == "Orange"{
            u_stock.orange_count =  u_stock.orange_count - count;
            website_inv.orange_stock =  website_inv.orange_stock + count;
        } else if fruit == "Banana"{
            u_stock.banana_count =  u_stock.banana_count - count;
            website_inv.banana_stock =  website_inv.banana_stock + count;
        } else if fruit == "Watermelon"{
            u_stock.watermelon_count =  u_stock.watermelon_count - count;
            website_inv.watermelon_stock =  website_inv.watermelon_stock + count;
        } else if fruit == "Grape"{
            u_stock.grape_count =  u_stock.grape_count - count;
            website_inv.grape_stock =  website_inv.grape_stock + count;
        }
        //3. Update website stock. Subtract traded fruit
        website_inv.broccoli_stock = website_inv.broccoli_stock - count; 

        Ok(Redirect::to("/tradesuccess"))
    }
}

#[get("/BroccoliTrade")]
async fn broccoli(user: Option<User>) -> Template {
    unsafe{
        Template::render("FoodPages/BroccoliPage", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[derive (Debug,FromForm)]
struct GrapeForm<'r> {
    grape_count: u32,
    fruit: &'r str,
}

#[post ("/GrapeTrade", data = "<s>")]
fn grapeform(s : Form<GrapeForm<'_>>) -> Result<Redirect, Error>{
    unsafe{
        println! ("received submission of {s:?}");
        let fruit = s.fruit;
        let count = s.grape_count;
        //1. Update the users stock. ADD
        u_stock.grape_count = u_stock.grape_count + count;
        //2. Update user stock. Subtract traded fruit from user stock. At same time, add to website inventory.
        if fruit == "Apple" {
            u_stock.apple_count =  u_stock.apple_count - count;
            website_inv.apple_stock =  website_inv.apple_stock + count;
        } else if fruit == "Orange"{
            u_stock.orange_count =  u_stock.orange_count - count;
            website_inv.orange_stock =  website_inv.orange_stock + count;
        } else if fruit == "Banana"{
            u_stock.banana_count =  u_stock.banana_count - count;
            website_inv.banana_stock =  website_inv.banana_stock + count;
        } else if fruit == "Watermelon"{
            u_stock.watermelon_count =  u_stock.watermelon_count - count;
            website_inv.watermelon_stock =  website_inv.watermelon_stock + count;
        } else if fruit == "Broccoli"{
            u_stock.broccoli_count =  u_stock.broccoli_count - count;
            website_inv.broccoli_stock =  website_inv.broccoli_stock + count;
        }
        //3. Update website stock. Subtract traded fruit
        website_inv.grape_stock = website_inv.grape_stock - count; 

        Ok(Redirect::to("/tradesuccess"))
    }
}

#[get("/GrapeTrade")]
async fn grape(user: Option<User>) -> Template {
    unsafe{
        Template::render("FoodPages/GrapePage", json!({ "user": user, "u_stock":u_stock }))
    }
}

#[get("/test")]
async fn test(user: Option<User>) -> Template {
    Template::render ("NewUser", context! { })
}


#[tokio::main]
async fn main() -> Result<(), Error> {

    
    let conn = SqlitePool::connect("database.db").await?;
    let users: Users = conn.clone().into();
    users.create_table().await?; 
    
    let _ = rocket::build().mount("/", 
    routes![index,get_login, post_login, sign_up_page, post_signup, test,logout, update, after_trade,
    apple,banana, watermelon, orange, broccoli, grape,
    formsub, appleform,bananaform, orangeform, watermelonform, broccoliform, grapeform]).mount("/", FileServer::from(relative!("static")))
    .manage(users)
    .manage(conn)
    .attach(Template::fairing()) //To use my created templates.
    .launch()
    .await
    .unwrap();
    Ok(())
}
