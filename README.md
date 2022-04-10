Discord Bot to Archive Messages
============

This bot is created to archive all messages. It will create a token to see and search this messages. 

!archive

Command can be used to trigger bot. It will send a message to same channel containing archive url and generated token.
This token has permission to see only this channels messages
Token has an expiry time.

!token

Command can be used to renew token.

Running
It is using meili search instance to archive all messages.


Adding Bot to Server
------------
https://canary.discord.com/api/oauth2/authorize?client_id=955569692171468850&permissions=84992&scope=bot

This link can be used to add bot to the server.

Since I am maintaining the server I can not guarantee this messages will stay forever.

Running Your Own Bot
------------

### 1.) Run Meilisearch Instance ###
Archivist archive messages to meili search. Meilisearch is a RESTful search API.

We need to run meili search instance to save messages. You can check options from document. 
https://docs.meilisearch.com/

To run meili search with docker for testing
~~~
docker run -d -it --rm -p 7700:7700 -v $(pwd)/data.ms:/data.ms getmeili/meilisearch:latest
~~~

This will create meili search instance without authentication

### 2.) Run Bot ###
4 main variable must be provided for bot. These variables can be on environment. You can also put them on .env file.
~~~
DISCORD_TOKEN=
MEILI_SEARCH_URL=
MEILI_SEARCH_MASTER_KEY=
MEILI_SEARCH_READ_TOKEN_TIMEOUT_IN_SECONDS=604800
~~~

DISCORD_TOKEN is your discord application token.

MEILI_SEARCH_URL is where your meili search instance is running like
MEILI_SEARCH_URL=http://localhost:7700/

MEILI_SEARCH_MASTER_KEY is your meili search master key. You can provide master key to meili search when running instance.
This key can be used later to read all messages or to create token.

MEILI_SEARCH_READ_TOKEN_TIMEOUT_IN_SECONDS this is timeout for created token default is 1 week.

### Run Bot with Docker ###
First build project to create .lock file 
~~~
cargo b --release
~~~

~~~
docker build -t archivist .
~~~

~~~
docker run -d --name archivist-bot --env-file .env archivist
~~~