use vk_bot::{
    keyboard::{Button, Color, Keyboard},
    Bot, Core, Event, Handler, Tester,
};

fn main() {
    // A simple closure for convenience...
    let simple_handler = |message| {
        // ...that returns a handler!
        Handler::new(move |ctx| {
            // This handler just sets the text of the message and sends it.
            ctx.response().set_message(message);
            eprintln!("{:?}", ctx.send());
        })
    };

    // Create a keyboard.
    let kbd = Keyboard::new(
        // Vec of rows
        vec![
            // Row 0
            vec![
                Button::new("A", Color::Primary, None),
                Button::new("B", Color::Default, Some(r#"{"a": "b"}"#.into())),
            ],
        ],
        false, // One-time? (i.e. show only until a button is pressed on the keyboard?)
    );

    let core = Core::new()
        .cmd_prefix("/")
        .cmd(
            "keyboard",
            Handler::new(move |ctx| {
                ctx.response().set_message("Here you go:");
                ctx.response().set_keyboard(kbd.clone());
                eprintln!("{:?}", ctx.send());
            }),
        )
        .regex("nice", simple_handler("Thanks!"))
        .on(Event::NoMatch, simple_handler("I don't understand..."))
        .payload(r#"{"a":"b"}"#, simple_handler("You pressed button B!"))
        // accept all remaining payloads!
        .dyn_payload(Tester::new(|_| true), simple_handler("Received a payload!"));

    Bot::new(
        "your vk token",      // VK token
        "f123456",            // Confirmation token (from Callback API settings)
        1,                    // Group ID
        "very_secure_phrase", // Secret (from Callback API settings)
        12345,                // Port
        core,
    )
    .start();
}
