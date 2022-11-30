// Spacing for my sanity



const { clientId, guildId, token, readRulesRoleId, gamersRoleId, masterKeyB4A} = require('./config.json');

//Creates PORT so that heroku can connect
var PORT = process.env.PORT || 5000;
var http = require('http');
http.createServer(function (req, res) {
	res.writeHead(200, {'Content-Type': 'text/plain'});
	res.write('Hello World!');
	res.end();
}).listen(PORT);
//Pings the website so it doesnt go offline
var http = require("http");
setInterval(function() {
    http.get("http://gamedavbot.herokuapp.com");
}, 300000); // every 5 minutes (300000)

// Global Variables and Requires
const { Client, Partials, GatewayIntentBits, SlashCommandBuilder, Routes, SystemChannelFlagsBitField, Role, Guild } = require('discord.js');
const { REST } = require('@discordjs/rest');
	// Allows for servers and server messages and server message reactions to be worked with; partials allow past data to be worked with 
const client = new Client({ intents: [GatewayIntentBits.Guilds, GatewayIntentBits.GuildMessages, GatewayIntentBits.GuildMessageReactions, GatewayIntentBits.GuildMembers], partials: [Partials.Message, Partials.Channel, Partials.Reaction] });
const fs = require('fs');
var throwable = false;

//The sleep function from python
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function moveBall(interaction, initial = true)
{

	// FOR THE THROW COMMAND IT WILL NEED TO DELETE THE USERID WHEN THROW IS CALLED
	// READ ME

	if(initial==true)
	{
		// THIS IS HOW TO SEND A MESSAGE
		const systemChannel = client.channels.cache.get(interaction.channelId);
		await interaction.reply('Now playing ball!');

		//Plays ball
		const ballStages = await systemChannel.send('o----\n-----\n-----');
		await sleep(500);
		ballStages.edit('-----\n-o---\n-----');
		await sleep(500);
		ballStages.edit('-----\n-----\n--o--');
		await sleep(500);
		ballStages.edit('-----\n---o-\n-----');
		await sleep(500);
		ballStages.edit('----o\n-----\n-----');
		await sleep(500);
		ballStages.react('🖐️');
		systemChannel.send('Nice catch! Now throw it back with /throw');
		return true;
	}
	else
	{
		// THIS IS HOW TO SEND A MESSAGE
		const systemChannel = client.channels.cache.get(interaction.channelId);
		await interaction.reply('*You passed it back*');

		//Plays ball
		const ballStages = await systemChannel.send('----o\n-----\n-----');
		await sleep(500);
		ballStages.edit('-----\n---o-\n-----');
		await sleep(500);
		ballStages.edit('-----\n-----\n--o--');
		await sleep(500);
		ballStages.edit('-----\n-o---\n-----');
		await sleep(500);
		ballStages.edit('o----\n-----\n-----');
		await sleep(500);
		ballStages.react('🖐️');
		systemChannel.send("Good throw! I'll bounce it back when you say /ball");
		return false;
	}
}


const commands = [
    // Uses the slash command builder to create new slash commands that have a .setName('') and .setDescription('')
	new SlashCommandBuilder().setName('ping').setDescription('Replies with pong!'),
	new SlashCommandBuilder().setName('ball').setDescription('Bounces a ball to you!'),
	new SlashCommandBuilder().setName('throw').setDescription('Throws the ball back!'),
]

//#region Formatting the commands to be put into discord
    //Maps commands to a discord json to send back
	//Command is each individual item in the list which is then made into a JSON
	.map(command => command.toJSON());

// I believe it connects the bot to the code using the bot's token
// it does this by creating a rest object
const rest = new REST({ version: '10' }).setToken(token);
// and then puts into the discord Guild Commands (with the client and guild id), the commands specified earlier. Also catches any errors.
rest.put(Routes.applicationGuildCommands(clientId, guildId), { body: commands })
	.then((data) => console.log(`Successfully registered ${data.length} application commands.`))
	.catch(console.error);
//#endregion

//#region Runs once when the application is ready
client.once('ready', async () => {
	console.log('Ready!');
});
//#endregion

//whenever something is interacted with, run the event listener code
client.on('interactionCreate', async interaction => {
    //Check if the interaction is a chat input command
	if (!interaction.isChatInputCommand()) return;

    // get the command name
	const { commandName } = interaction;

    //check which command and output; throwable bool is to see if the throw command should work
	//Ping command
	if (commandName === 'ping') {
		await interaction.reply('Pong!');
	} 
	// Ball command
	else if (commandName === 'ball') {
		if(throwable==false) {
			throwable = await moveBall(interaction);
		}
		else{
			await interaction.reply("I don't have ball back yet!");
		}
	} 
	// Throw command
	else if (commandName === 'throw') {
		if(throwable==true) {
			throwable = await moveBall(interaction, false);
		} else {
			await interaction.reply("You haven't been thrown the ball yet!");
		}
	}
});

client.on('guildMemberAdd', async member => {
	console.log("New Person Joined!");
    await member.roles.add(readRulesRoleId);
	console.log("Assigned Role");
});

client.on('messageReactionAdd', async (reaction_orig, user) => {
	// fetch the message if it's not cached
	const message = !reaction_orig.message.author
		? await reaction_orig.message.fetch()
		: reaction_orig.message;

	if (message.author.id === user.id) {
		// the reaction is coming from the same user who posted the message
		//return;
	}
		
	// the reaction is coming from a different user
	if(reaction_orig.emoji.name=="👍")
	{
		const guild = client.guilds.cache.get(guildId);
		const member = guild.members.cache.get(user.id);

		if(member.roles.cache.get(readRulesRoleId))
		{
			member.roles.remove(readRulesRoleId);
			member.roles.add(gamersRoleId);
			//Dm the user
			user.send("You now have access to the CGC Discord! Thanks for deciding to join the community!").catch(error => {
				//If you cant dm the user message the server
				client.channels.cache.get("1019776724004778024").send("Welcome " + user.username +"!");
			});
		}
	}
});

client.login(token);