# Serde Discord

serde_discord is a Rust library that simplifies interactions with the Discord API. This crate provides functionality to register commands, handle incoming interactions, and generate appropriate responses for various Discord interactions.

## Features
- Command Registration: Easily register Discord slash commands.
- Interaction Handling: Process and respond to incoming user interactions.
- Response Building: Build and send various response types, such as messages, modals, or autocompletes.
- Type Definitions: Provides convenient Rust types and enums for commands and options.

## Installation

To use this crate, add the following to your Cargo.toml:
```toml
[dependencies]
serde_discord = "0.1.0"  # Replace with the latest version
```

If you are using specific features, make sure to enable them in your Cargo.toml:
```toml
[dependencies]
serde_discord = { version = "0.1.0", features = ["interaction", "register", "response", "types"] }
```

## Usage

### 1. Command Registration Example
Here is an example of registering a simple command with the Discord API:
```rust
use std::env;
use serde_discord::{
    register::{register_commands, ChoiceValue, Command, CommandBuilder, CommandOptionBuilder},
    types::{CommandKind, CommandOptionKind},
};

fn make_cmd_roll() -> Command {
    let opt_min = CommandOptionBuilder::new()
        .name("min")
        .description("The mininum number to roll")
        .kind(CommandOptionKind::Integer)
        .min_value(ChoiceValue::Int(0))
        .max_value(ChoiceValue::Int(i32::MAX - 1))
        .build()
        .unwrap();

    let opt_max = CommandOptionBuilder::new()
        .name("max")
        .description("The max number to roll")
        .kind(CommandOptionKind::Integer)
        .min_value(ChoiceValue::Int(1))
        .max_value(ChoiceValue::Int(i32::MAX))
        .build()
        .unwrap();

    CommandBuilder::new()
        .name("roll")
        .kind(CommandKind::ChatInput)
        .description("Roll a die")
        .options(vec![opt_min, opt_max])
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();  // Load environment variables

    let cmd_roll = make_cmd_roll();
    let cmds = vec![cmd_roll];

    let app_id = env::var("DISCORD_APP_ID").expect("DISCORD_APP_ID not set");
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    register_commands(&app_id, &token, cmds).await.unwrap();
}
```

### 2. Handling Incoming Interactions

This crate allows you to process and respond to interactions from Discord with various types of responses like messages or autocompletes. Simply import the modules required for your use case.

## Environment Variables

The following environment variables are required to interact with the Discord API:

- `DISCORD_APP_ID`: Your application's ID.
- `DISCORD_TOKEN`: Your Discord bot token.

These variables can be set using `.env` files or exported directly in your environment.

## Features

- Default features: The types module is enabled by default, providing core type definitions for working with Discord commands and interactions.
- Optional features: Additional functionality can be enabled by specifying feature flags in your Cargo.toml. Depending on your use case, you can choose the appropriate modules:
```toml
[features]
default = ["types"]
register = ["types"]    # For command registration functionality
response = ["types"]    # For building and sending interaction responses
interaction = ["types"] # For processing incoming interactions
responding = ["types", "response", "interaction"]    # Convenience feature for responding to interactions
types = []  # Core types used across the library
```
- `register`: Use this feature if you only need to register commands with Discord.
- `responding`: This feature includes everything necessary for handling and responding to interactions, including both response-building and interaction-handling modules.

Typically, you only need register for command registration, and responding for handling and replying to interactions.

## API Versioning

This crate uses Discord API v10. If the API version changes, update the DISCORD_API_VERSION constant in your project to ensure compatibility.
```rust
pub const DISCORD_API_VERSION: &str = "v10";
```

## Contributing

Every interaction is not yet implemented and are either marked with `#[non_exhaustive]` or `TODO` comments. Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

