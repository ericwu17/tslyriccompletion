# Taylor Swift Lyric Completion In Rust

This is a fun project for fellow Taylor Swift fans who like to complete lyrics.
The project was originally a
[command line interface written in Python](https://github.com/EricWu2003/Taylor-Lyric-Guessing-Game).
Then it became a command line interface written in Rust (see the releases on this Github page).
It has since turned into a web app built with React and Rust!

The project is now live at [http://tslyriccompletion.com](http://tslyriccompletion.com)!
Thanks to David from the Linux Users group at UCLA for helping me get this project deployed
and answering my questions about things like apache and ports.

## Running the project locally

To run the code locally after cloning this repo, you will first need to set up a mysql database
manager. You will need to create a database called `mydb`, which can be done by running:

```bash
mysql -u root -p
CREATE DATABASE mydb
```

and then:

```bash
mysql -u root -p mydb < ./mydb_dump.sql
```

where `mydb_dump.sql` is the sql dump file which can be found in the server directory.

You will also need to set up .env files in both the client and server directory. The .env file
in the server directory will contain information about database credentials (see the README
file in the sever directory). The .env file in the client directory will contain something like:

```text
REACT_APP_AXIOS_BASE_URL=http://localhost:3000
```

The .env file is needed in the client because when I deploy this app at
[http://tslyriccompletion.com](http://tslyriccompletion.com), I instead include:

```text
REACT_APP_AXIOS_BASE_URL=http://tslyriccompletion.com/api/
```

Then, navigate to the `server` directory and run `cargo run`. In a separate terminal,
navigate to the client directory and run `npm install` followed by `npm start`

Please don't hesitate to contact me if these instructions are unclear. They're only meant to
help me remember how to run this app in the future. I'd love to try and help
if you're trying to run this app on your local machine!
