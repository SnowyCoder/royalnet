// See the following link for an example of how to use this file:
// https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/dispatching_features.rs

use anyhow::{Context, Error, Result};
use teloxide::{Bot, dptree};
use teloxide::dispatching::{DefaultKey, Dispatcher, HandlerExt, UpdateFilterExt};
use teloxide::dptree::entry;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::{ChatId, Message, MessageId, Update};
use teloxide::utils::command::BotCommands;

mod start;
mod fortune;
mod echo;
mod help;
mod whoami;
mod answer;
mod reminder;

#[derive(Debug, Clone, PartialEq, Eq, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
	#[command(description = "Invia messaggio di introduzione.")]
	Start,
	#[command(description = "Visualizza l'elenco dei comandi disponibili, o mostra informazioni su uno specifico comando.")]
	Help(String),
	#[command(description = "Mostra il tuo oroscopo di oggi.")]
	Fortune,
	#[command(description = "Ripeti il testo inviato.")]
	Echo(String),
	#[command(description = "Controlla a che account RYG è associato il tuo account Telegram.")]
	WhoAmI,
	#[command(description = "Rispondi ad una domanda.")]
	Answer(String),
	#[command(description = "Ricorda la chat di qualcosa che avverrà in futuro.")]
	Reminder(reminder::ReminderArgs),
}

impl Command {
	pub async fn set_commands(bot: &mut Bot) -> Result<()> {
		log::trace!("Determining bot commands...");
		let commands = Self::bot_commands();

		log::trace!("Setting commands on {bot:?}: {commands:#?}");
		let reply = bot.set_my_commands(commands).await
			.context("Impossibile aggiornare l'elenco comandi del bot.")?;

		log::trace!("Setting commands on {bot:?} successful: {reply:#?}");
		Ok(())
	}
}

async fn handle_command(bot: Bot, command: Command, message: Message) -> CommandResult {
	log::trace!("Received command: {command:?}");

	let result = match command {
		Command::Start => start::handler(&bot, &message).await,
		Command::Help(target) => match target.as_str() {
			"" => help::handler_all(&bot, &message).await,
			_ => help::handler_specific(&bot, &message, &target).await,
		},
		Command::Fortune => fortune::handler(&bot, &message).await,
		Command::Echo(text) => echo::handler(&bot, &message, &text).await,
		Command::WhoAmI => whoami::handler(&bot, &message).await,
		Command::Answer(_) => answer::handler(&bot, &message).await,
		Command::Reminder(args) => reminder::handler(&bot, &message, args).await,
	};

	if result.is_ok() {
		return Ok(())
	}

	let chat_id = message.chat.id;
	let message_id = message.id;
	let error = result.unwrap_err();

	let result2 = error_command(&bot, chat_id, message_id, &error).await;

	if result2.is_ok() {
		return Ok(())
	}

	let error2 = result2.unwrap_err();

	log::error!("Command message {message_id:?} in {chat_id:?} errored out with `{error}`, and it was impossible to handle the error because of `{error2}`\n\n{error2:?}");

	Ok(())
}

async fn error_command(bot: &Bot, chat_id: ChatId, message_id: MessageId, error: &Error) -> CommandResult {
	log::debug!("Command message {message_id:?} in {chat_id:?} errored out with `{error}`");

	let text = format!("⚠️ {error}");

	let _reply = bot
		.send_message(chat_id, text)
		.reply_to_message_id(message_id)
		.await
		.context("Non è stato possibile inviare la risposta.")?;

	Ok(())
}

async fn unknown_command(bot: Bot, message: Message) -> CommandResult {
	log::debug!("Received an unknown command.");

	bot.send_message(message.chat.id, "⚠️ Comando sconosciuto.")
		.reply_to_message_id(message.id)
		.await
		.context("Non è stato possibile inviare la risposta.")?;

	Ok(())
}

pub fn dispatcher(bot: Bot) -> Dispatcher<Bot, Error, DefaultKey> {
	Dispatcher::builder(
		bot,
		Update::filter_message()
			.branch(
				entry()
					.filter_command::<Command>()
					.endpoint(handle_command)
			)
			.endpoint(unknown_command)
	)
		.dependencies(
			dptree::deps![]  // No deps needed at the moment.
		)
		.build()
}

type CommandResult = anyhow::Result<()>;