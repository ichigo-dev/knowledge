import sys
import os
import boto3
from botocore.exceptions import ClientError
import base64
import mysql.connector
import json

#   RDS settings
DB_HOST = os.environ['DB_HOST']
DB_NAME = os.environ['DB_NAME']

def lambda_handler(event, context):
    secret = get_secret()
    try:
        conn = mysql.connector.connect(
            host=DB_HOST,
            user=secret['username'],
            password=secret['password'],
            database=DB_NAME)
    except Exception  as e:
        print("Fail connecting to RDS mysql instance")
        print(e)
        sys.exit()
 
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

def get_secret():
    secret_name = "ichigo-dev-knowledge-rds-secret"
    region_name = "ap-northeast-1"

    session = boto3.session.Session()
    client = session.client(
        service_name='secretsmanager',
        region_name=region_name
    )

    try:
        get_secret_value_response = client.get_secret_value(
            SecretId=secret_name
        )
    except ClientError as e:
        raise e

    secret = get_secret_value_response['SecretString']
    return json.loads(secret)
