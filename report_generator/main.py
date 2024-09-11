import datetime
import mysql.connector
import smtplib
from email.message import EmailMessage

ANON_PLAYER = "<anon>"

def find_median(a, func):
    if len(a) == 0:
        return None
    a.sort(key=func)
    if len(a) % 2 == 1:
        return func(a[len(a)//2])
    else:
        return (func(a[len(a)//2]) + func(a[(len(a)-1)//2])) / 2


db_user = ""
db_pw = ""

with open("./.env") as f:
    for line in f.readlines():
        if line.startswith("DATABASE_USER="):
            db_user = line.strip().strip("DATABASE_USER=")
        if line.startswith("DATABASE_PASSWORD="):
            db_pw = line.strip().strip("DATABASE_PASSWORD=")


cnx = mysql.connector.connect(user=db_user, password=db_pw, database='mydb')
cursor = cnx.cursor(buffered=True, dictionary=True)

query_select_all_from_games = ("SELECT uuid, terminal_score, has_terminated, player_name FROM games "
         "WHERE start_time BETWEEN %s AND %s")
query_select_all_from_guesses = ("SELECT * FROM guesses "
         "WHERE submit_time BETWEEN %s AND %s")

gen_time = datetime.datetime.now()
end_time = datetime.datetime.now().replace(day=1, hour=0, minute=0, second=0, microsecond=0)
start_time = (end_time - datetime.timedelta(days=15)).replace(day=1, hour=0, minute=0, second=0, microsecond=0)


cursor.execute(query_select_all_from_games, (start_time, end_time))
games = {}
total_score = 0
incomplete_games = 0
complete_games = 0
total_games = 0
for row in cursor:
    total_games += 1
    row["num_guesses"] = 0
    games[row["uuid"]] = row
    if row["has_terminated"]:
        total_score += row["terminal_score"]
        complete_games += 1
    else:
        incomplete_games += 1



cursor.execute(query_select_all_from_guesses, (start_time, end_time))
guesses = []
for row in cursor:
    guesses.append(row)


# compute number of guesses for each game
for guess in guesses:
    game_id = guess["game_uuid"]
    if game_id in games.keys():
        games[game_id]["num_guesses"] += 1
games = list(games.values())
games.sort(key=lambda g: g["num_guesses"])


terminated_games = list(filter(lambda g: g["has_terminated"], games))
unterminated_games = list(filter(lambda g: not g["has_terminated"], games))


total_guesses = len(guesses)

avg_guesses_per_game = total_guesses / total_games
med_guesses_per_game = find_median(terminated_games, lambda g: g["num_guesses"])
terminated_games.sort(key=lambda g: g["num_guesses"])
max_guesses_per_game = terminated_games[-1]["num_guesses"]
max_guesses_per_game_player = terminated_games[-1]["player_name"]


avg_score_game = total_score / total_games
med_score_game = find_median(terminated_games, lambda g: g["terminal_score"])
terminated_games.sort(key=lambda g: g["terminal_score"])
max_score_game = terminated_games[-1]["terminal_score"]
max_score_game_player = terminated_games[-1]["player_name"]

terminated_games.sort(key=lambda g: g["terminal_score"], reverse=True)
for game in terminated_games:
    if game["player_name"] is None:
        game["player_name"] = ANON_PLAYER

unterminated_games.sort(key=lambda g: g["num_guesses"], reverse=True)

terminal_guesses = list(filter(lambda g:g["result"] == "incorrect" and g["options"] == "[]", guesses))


FINAL_RESULT = {
    "generation_time": gen_time.strftime("%Y-%m-%d %H:%M:%S"),
    "start_time": start_time.strftime("%Y-%m-%d"),
    "end_time": end_time.strftime("%Y-%m-%d"),

    "total_games": str(total_games),
    "total_guesses": str(total_guesses),
    "incomplete_games": str(incomplete_games),
    "complete_games": str(complete_games),

    "avg_guesses_per_game": "{:.2f}".format(avg_guesses_per_game),
    "med_guesses_per_game": "{:.2f}".format(med_guesses_per_game),
    "max_guesses_per_game": str(max_guesses_per_game),
    "max_guesses_per_game_player": str(max_guesses_per_game_player) if max_guesses_per_game_player is not None else ANON_PLAYER,

    "avg_score_game": "{:.2f}".format(avg_score_game),
    "med_score_game": "{:.2f}".format(med_score_game),
    "max_score_game": str(max_score_game),
    "max_score_game_player": str(max_score_game_player)  if max_score_game_player is not None else ANON_PLAYER,

    "terminated_games": terminated_games,
    "unterminated_games": unterminated_games,

    "terminal_guesses": terminal_guesses,
    "num_terminal_guesses": str(len(terminal_guesses)),
}


report_output_lines = []


with open("report_template.txt") as f:
    while True:
        line = f.readline()
        if not line:
            break

        if line.startswith("*REPEAT*"):
            repeat_arr_key = line.split("$")[1]
            repeat_arr = FINAL_RESULT[repeat_arr_key]
            repeat_text = ""
            while True:
                next_line = f.readline()
                if next_line.startswith("*END_REPEAT*"):
                    break
                else:
                    repeat_text += next_line
            for elem in repeat_arr:
                t = repeat_text
                for key, val in elem.items():
                    t = t.replace(f"${key}$", str(val))
                report_output_lines.append(t)


            continue

        while "$" in line:
            key = line.split("$")[1]
            line = line.replace(f"${key}$", FINAL_RESULT[key])

        report_output_lines.append(line)


final_report_string = "".join(report_output_lines)



cursor.close()
cnx.close()




## SEND EMAIL HERE

with open("./.env") as f:
    for line in f.readlines():
        if line.startswith("EMAIL_ADDRESS="):
            EMAIL_ADDRESS = line.strip().strip("EMAIL_ADDRESS=")
        if line.startswith("EMAIL_PASS="):
            EMAIL_PASSWORD = line.strip().strip("EMAIL_PASS=")
        if line.startswith("EMAIL_RECIPIENT="):
            EMAIL_RECIPIENT = line.strip().strip("EMAIL_RECIPIENT=")



msg = EmailMessage()
msg['Subject'] = f"""TSLC Monthly Report {end_time.strftime("%Y-%m-%d")}"""
msg['From'] = EMAIL_ADDRESS
msg['To'] = EMAIL_RECIPIENT
msg.set_content('Please find the report attached.\n')
msg.add_attachment(final_report_string.encode("utf-8"), maintype='application', subtype='octet-stream', filename="report.txt")

with smtplib.SMTP_SSL('smtp.gmail.com', 465) as smtp:
    smtp.login(EMAIL_ADDRESS, EMAIL_PASSWORD) 
    smtp.send_message(msg)
