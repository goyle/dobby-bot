# dobby-bot
A Discord bot designed to provide instant answers to [Grin](https://github.com/mimblewimble/grin/) related questions.

## Usage
Prepend all commands with the prefix `GRIN `.

### Project
| Command           | Aliases                                                          |
| ----------------- | ---------------------------------------------------------------- |
| `code_of_conduct` | `coc` `CoC` `COC` `code` `conduct` `codeofconduct`               |
| `coin`            | `info` `grininfo` `grinfo`                                       |
| `emission`        | `emissions` `blocktime` `reward` `miningreward` `monetarypolicy` |
| `governance`      | `gov`                                                            |
| `philosophy`      | `philo`                                                          |
| `roadmap`         |                                                                  |
| `symbol`          | `currency` `currencysymbol`                                      |

### Community
| Command           | Aliases                                                          |
| ----------------- | ---------------------------------------------------------------- |
| `explorers`       | `explorer`                                                       |
| `spending_log`    | `spending` `spendinglog`                                         |
| `wallets`         | `wallet`                                                         |

### Mining
| Command           | Aliases                                                          |
| ----------------- | ---------------------------------------------------------------- |
| `calculator`      | `calc` `miningcalc` `miningcalculator`                           |
| `miners`          | `miner`                                                          |

## Build, Configuration, and Running

### Requirements
* Rust: Install using rustup: https://rustup.rs
* A Discord token

### Getting a Discord token
1. Create a [Discord application](https://discordapp.com/developers/applications/).
2. Go to your application and then to the Bot tab under Settings. In here, you can create a bot and copy the token. The token is what will be put in the `DISCORD_TOKEN` variable inside the `.env` file.
3. Create the invite link for the bot by going to the OAuth2 tab under Settings. Select the checkbox for bot in Scopes. The invite link should appear. Select any checkbox for permissions you want the bot to have.

### Build and run
1. Clone the repository: `git clone https://github.com/goyle/dobby-bot.git`
2. Open `.env` and insert your Discord bot token.
3. Run the bot: `cargo run --release`
