from utils import utils
import mysql.connector
import json

def lambda_handler(event, context):
    try:
        conn = utils.rds_connect()
    except Exception as e:
        print(e)
        return {
            'statusCode': 400,
            'body': 'Bad Request'
        }
 
    cursor = conn.cursor(dictionary=True)
    sql = "SELECT * FROM `term` WHERE `deleted` = 0 ORDER BY rand() LIMIT 1"
    cursor.execute(sql)
    result = cursor.fetchone()
    cursor.close()
    conn.close()

    return {
        'statusCode': 200,
        'body': json.dumps(result, indent=2, default=str, ensure_ascii=False)
    }
