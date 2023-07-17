import sys
import os
import boto3
from botocore.exceptions import ClientError
import base64
import mysql.connector
import json


def rds_connect():
    db_host = os.environ['DB_HOST']
    db_name = os.environ['DB_NAME']

    secret = get_rds_secret()
    try:
        conn = mysql.connector.connect(
            host=db_host,
            user=secret['username'],
            password=secret['password'],
            database=db_name)
    except Exception as e:
        print("Fail connecting to RDS mysql instance")
        raise e

    return conn


def get_rds_secret():
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
