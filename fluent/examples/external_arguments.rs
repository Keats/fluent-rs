extern crate fluent;

use fluent::context::MessageContext;
use fluent::types::FluentValue;
use std::collections::HashMap;

fn main() {
    let mut ctx = MessageContext::new(&["en"]);

    ctx.add_messages(
        "
hello-world = Hello { $name }
ref = The previous message says { hello-world }
unread-emails =
    { $emailCount ->
        [one] You have { $emailCount } unread email
       *[other] You have { $emailCount } unread emails
    }
",
    );

    let mut args = HashMap::new();
    args.insert("name", FluentValue::from("John"));

    match ctx
        .get_message("hello-world")
        .and_then(|msg| ctx.format(msg, Some(&args)))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    match ctx
        .get_message("ref")
        .and_then(|msg| ctx.format(msg, Some(&args)))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    let mut args = HashMap::new();
    args.insert("emailCount", FluentValue::as_number("1.0").unwrap());

    match ctx
        .get_message("unread-emails")
        .and_then(|msg| ctx.format(msg, Some(&args)))
    {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
}
