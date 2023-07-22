from utils import utils
import mysql.connector
import json

def lambda_handler(event, context):
    try:
        user_id = event['pathParameters']['user_id']
        conn = utils.rds_connect()
    except Exception as e:
        print(e)
        return {
            'statusCode': 400,
            'body': 'Bad Request'
        }

    conn.start_transaction()
    cursor = conn.cursor(dictionary=True)
    sql = "SELECT * FROM `user_result` WHERE `user_id` = " + str(user_id)
    sql += " ORDER BY `created_at` desc"
    cursor.execute(sql)
    result = cursor.fetchall()
    for row in result:
        print(row)
        sql = "SELECT * FROM `user_answer` WHERE `user_result_id` = " + str(row["user_result_id"])
        cursor.execute(sql)
        row["answers"] = cursor.fetchall()

        for answer in row["answers"]:
            sql = "SELECT * FROM `term` WHERE `term_id` = " + str(answer["term_id"])
            cursor.execute(sql)
            answer["term"] = cursor.fetchone()
    cursor.close()
    conn.commit()
    conn.close()

    return {
        'statusCode': 200,
        'body': json.dumps(result, indent=2, default=str, ensure_ascii=False)
    }
