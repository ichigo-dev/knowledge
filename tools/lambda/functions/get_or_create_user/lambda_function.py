from utils import utils
import mysql.connector
import json

def lambda_handler(event, context):
    try:
        code = event['pathParameters']['code']
        conn = utils.rds_connect()
    except Exception  as e:
        print(e)
        return {
            'statusCode': 400,
            'body': 'Bad Request'
        }
 
    cursor = conn.cursor(dictionary=True)
    sql = "SELECT * FROM `user` WHERE `code` = '" + code + "'"
    cursor.execute(sql)
    result = cursor.fetchone()
    if result is None:
        sql = "INSERT INTO `user` (`code`) VALUES ('" + code + "')"
        cursor.execute(sql)
        result = cursor.fetchone()
    cursor.close()
    conn.commit()
    conn.close()

    return {
        'statusCode': 200,
        'body': json.dumps(result, indent=2, default=str, ensure_ascii=False)
    }
