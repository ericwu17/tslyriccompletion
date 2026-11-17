
# taylor swift lyric completion server

The server connects to a mysql database running on the local machine. The database's name is `mydb`.
A .env file is required in this directory. The .env file contains contents of the form:

```text
DATABASE_USER=root
DATABASE_PASSWORD=secret_here

# Used to send account emails (verification, password reset) through Gmail SMTP
EMAIL_FROM=address@gmail.com
EMAIL_PASS=app_password_here

# Base URL used in links inside those emails (defaults to http://localhost:3000)
FRONTEND_URL=https://tslyriccompletion.com
```

Only the database variables are required for the server to start. Without the email variables,
signup (with an email address), email verification, and password reset requests will fail.

The sql server is backed up periodically (weekly) by using the mysqldump command.
A cron job is used to execute this command. Here is the cron file:

```cron
58 1 * * 1 mysqldump mydb | gzip > ~/backups/mydb--$(date +\%Y-\%m-\%d--\%H-\%M)-00.sql.gz
# The line above creates a database backup every monday at 1:58 AM
# The time is a reference to the song Last Kiss
```

To recover from one of the backup files, first extract the tarball, and then use the command

```bash
mysql mydb < file.sql
```

OR

```bash
mysql -u root -p mydb < file.sql
```

where `file.sql` is the filename (after extraction). Using the -p option will cause mysql to prompt
you for a password.
