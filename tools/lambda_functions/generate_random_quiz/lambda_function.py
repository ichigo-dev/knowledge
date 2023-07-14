import mysql.connector
import json
 
#   RDS settings
DB_USER = "qzgen"
DB_PORT = 3306
DB_PASSWORD = "Y77rZ21mHyMG9tagC^n7"
DB_HOST = "ichigo-dev-knowledge-db.c5ecesxssz8k.ap-northeast-1.rds.amazonaws.com"
DB_NAME = "qzgen"

def lambda_handler(event, context):
    try:
        conn = mysql.connector.connect(
            host=DB_HOST,
            port=DB_PORT,
            user=DB_USER,
            password=DB_PASSWORD,
            database=DB_NAME)
    except Exception  as e:
        print("Fail connecting to RDS mysql instance")
        print(e)
        sys.exit()
 
    cursor = conn.cursor(dictionary=True)
    sql = "SELECT * FROM `term` ORDER BY rand() LIMIT 1"
    cursor.execute(sql)
    result = cursor.fetchone()
    cursor.close()
    conn.close()

    return {
        'statusCode': 200,
        'body': json.dumps(result, indent=2, default=str)
    }
