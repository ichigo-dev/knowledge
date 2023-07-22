from utils import utils
import mysql.connector
import base64
import json

def lambda_handler(event, context):
    try:
        body = json.loads(base64.b64decode(event['body']).decode())
        conn = utils.rds_connect()
    except Exception  as e:
        print(e)
        return {
            'statusCode': 400,
            'body': 'Bad Request'
        }
 
    conn.start_transaction()
    cursor = conn.cursor(dictionary=True)
    sql = "INSERT INTO `user_answer` (`user_result_id`, `term_id`, `is_correct`) VALUES ("
    sql += str(body["user_result_id"]) + ", "
    sql += str(body["term_id"]) + ", "
    sql += str(1 if body["is_correct"] else 0) + ")"
    cursor.execute(sql)
    sql = "SELECT * FROM `user_answer` WHERE `user_answer_id` = " + str(cursor.lastrowid)
    cursor.execute(sql)
    result = cursor.fetchone()
    cursor.close()
    conn.commit()
    conn.close()

    return {
        'statusCode': 200,
        'body': json.dumps(result, indent=2, default=str, ensure_ascii=False)
    }
