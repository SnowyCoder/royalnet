#![allow(unused_imports)]

use std::io::Write;

use diesel::deserialize::FromSql;
use diesel::serialize::ToSql;

mod users;
mod telegram;
mod discord;
mod steam;
mod brooch_match;
mod diario;
mod matchmaking_events;
mod matchmaking_replies;
mod matchmaking_messages_telegram;
mod matchmaking_choice;

pub use users::{RoyalnetUser, RoyalnetUserId};
pub use telegram::{TelegramUser, TelegramChatId, TelegramMessageId, TelegramUserId};
pub use discord::{DiscordUser, DiscordUserId};
pub use steam::{SteamUser, SteamId64};
pub use brooch_match::{BroochMatch, DotaMatchId};
pub use diario::{Diario, DiarioId};
pub use matchmaking_events::{MatchmakingEvent, MatchmakingId};
pub use matchmaking_replies::MatchmakingReply;
pub use matchmaking_messages_telegram::{MatchmakingMessageTelegram, telegram_ext::MatchmakingTelegramKeyboardCallback};
pub use matchmaking_choice::MatchmakingChoice;
