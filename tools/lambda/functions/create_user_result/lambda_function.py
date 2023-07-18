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
            'body': json.dumps('Bad Request')
        }

    conn.start_transaction()
    cursor = conn.cursor(dictionary=True)
    sql = "INSERT INTO `user_result` (`user_id`) VALUES (" + str(user_id) + ")"
    cursor.execute(sql)
    sql = "SELECT * FROM `user_result` WHERE `user_result_id` = " + str(cursor.lastrowid)
    cursor.execute(sql)
    result = cursor.fetchone()
    cursor.close()
    conn.commit()
    conn.close()

    return {
        'statusCode': 200,
        'body': json.dumps(result, indent=2, default=str, ensure_ascii=False)
    }
