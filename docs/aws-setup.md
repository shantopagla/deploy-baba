# AWS Setup Guide

This guide walks you through configuring AWS access for deploying deploy-baba.

## 1. Create an AWS Profile

Add this to `~/.aws/config`:

```ini
[profile deploy-baba]
region = us-east-1
output = json
```

For access key authentication, add credentials to `~/.aws/credentials`:

```ini
[deploy-baba]
aws_access_key_id = YOUR_KEY
aws_secret_access_key = YOUR_SECRET
```

For SSO users, configure SSO fields in the profile section instead.

## 2. Required IAM Permissions

Attach this policy to the IAM user/role used for deployment. It covers both
CI/CD operations and full Terraform provisioning (first-time `just infra-apply`).

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "STSValidation",
      "Effect": "Allow",
      "Action": ["sts:GetCallerIdentity"],
      "Resource": "*"
    },
    {
      "Sid": "SSMAccess",
      "Effect": "Allow",
      "Action": ["ssm:GetParameter", "ssm:PutParameter", "ssm:DeleteParameter", "ssm:DescribeParameters"],
      "Resource": "arn:aws:ssm:*:*:parameter/deploy-baba/*"
    },
    {
      "Sid": "ECRPublicPush",
      "Effect": "Allow",
      "Action": ["ecr-public:*"],
      "Resource": "*"
    },
    {
      "Sid": "LambdaFullLifecycle",
      "Effect": "Allow",
      "Action": [
        "lambda:CreateFunction", "lambda:DeleteFunction", "lambda:GetFunction",
        "lambda:UpdateFunctionCode", "lambda:UpdateFunctionConfiguration",
        "lambda:PublishVersion", "lambda:AddPermission", "lambda:RemovePermission",
        "lambda:CreateFunctionUrlConfig", "lambda:UpdateFunctionUrlConfig",
        "lambda:GetFunctionUrlConfig", "lambda:ListFunctions"
      ],
      "Resource": "arn:aws:lambda:*:*:function:deploy-baba-*"
    },
    {
      "Sid": "IAMRolesForTerraform",
      "Effect": "Allow",
      "Action": [
        "iam:CreateRole", "iam:DeleteRole", "iam:GetRole", "iam:ListRoles",
        "iam:AttachRolePolicy", "iam:DetachRolePolicy", "iam:PutRolePolicy",
        "iam:DeleteRolePolicy", "iam:GetRolePolicy", "iam:ListRolePolicies",
        "iam:ListAttachedRolePolicies", "iam:PassRole", "iam:TagRole", "iam:UntagRole"
      ],
      "Resource": "arn:aws:iam::*:role/deploy-baba-*"
    },
    {
      "Sid": "S3StateAndBackups",
      "Effect": "Allow",
      "Action": [
        "s3:CreateBucket", "s3:DeleteBucket", "s3:GetBucketLocation",
        "s3:ListBucket", "s3:GetBucketVersioning", "s3:PutBucketVersioning",
        "s3:GetBucketPolicy", "s3:PutBucketPolicy", "s3:DeleteBucketPolicy",
        "s3:GetObject", "s3:PutObject", "s3:DeleteObject",
        "s3:GetEncryptionConfiguration", "s3:PutEncryptionConfiguration"
      ],
      "Resource": [
        "arn:aws:s3:::deploy-baba-*",
        "arn:aws:s3:::deploy-baba-*/*"
      ]
    },
    {
      "Sid": "DynamoDBTerraformLock",
      "Effect": "Allow",
      "Action": [
        "dynamodb:CreateTable", "dynamodb:DeleteTable", "dynamodb:DescribeTable",
        "dynamodb:GetItem", "dynamodb:PutItem", "dynamodb:DeleteItem"
      ],
      "Resource": "arn:aws:dynamodb:*:*:table/terraform-lock"
    },
    {
      "Sid": "EFSForSQLite",
      "Effect": "Allow",
      "Action": [
        "elasticfilesystem:CreateFileSystem", "elasticfilesystem:DeleteFileSystem",
        "elasticfilesystem:DescribeFileSystems",
        "elasticfilesystem:CreateMountTarget", "elasticfilesystem:DeleteMountTarget",
        "elasticfilesystem:DescribeMountTargets",
        "elasticfilesystem:CreateAccessPoint", "elasticfilesystem:DeleteAccessPoint",
        "elasticfilesystem:DescribeAccessPoints",
        "elasticfilesystem:ClientMount", "elasticfilesystem:ClientWrite",
        "elasticfilesystem:PutLifecycleConfiguration", "elasticfilesystem:TagResource"
      ],
      "Resource": "*"
    },
    {
      "Sid": "EC2VPCForLambdaAndEFS",
      "Effect": "Allow",
      "Action": [
        "ec2:DescribeVpcs", "ec2:DescribeSubnets", "ec2:DescribeSecurityGroups",
        "ec2:CreateSecurityGroup", "ec2:DeleteSecurityGroup",
        "ec2:AuthorizeSecurityGroupIngress", "ec2:AuthorizeSecurityGroupEgress",
        "ec2:RevokeSecurityGroupIngress", "ec2:RevokeSecurityGroupEgress",
        "ec2:DescribeNetworkInterfaces", "ec2:CreateNetworkInterface",
        "ec2:DeleteNetworkInterface", "ec2:DescribeAvailabilityZones"
      ],
      "Resource": "*"
    },
    {
      "Sid": "CloudWatchLogs",
      "Effect": "Allow",
      "Action": [
        "logs:CreateLogGroup", "logs:DeleteLogGroup", "logs:DescribeLogGroups",
        "logs:CreateLogStream", "logs:PutLogEvents", "logs:GetLogEvents",
        "logs:FilterLogEvents", "logs:PutRetentionPolicy"
      ],
      "Resource": "arn:aws:logs:*:*:log-group:/aws/lambda/deploy-baba-*"
    },
    {
      "Sid": "EventBridgeBackupSchedule",
      "Effect": "Allow",
      "Action": [
        "events:CreateRule", "events:DeleteRule", "events:DescribeRule",
        "events:PutRule", "events:PutTargets", "events:RemoveTargets",
        "events:ListTargetsByRule", "events:TagResource"
      ],
      "Resource": "arn:aws:events:*:*:rule/deploy-baba-*"
    }
  ]
}
```

## 3. Bootstrap (First Time Only)

```bash
just infra-bootstrap deploy-baba
```

This creates the S3 state bucket for Terraform and writes the SSM sentinel
parameter that `just aws-check` uses for validation.

## 4. Validate Setup

```bash
just aws-check deploy-baba
```

Expected output:
```
✓ AWS profile 'deploy-baba' is configured correctly
✓ Account: 123456789012, Region: us-east-1
```

## 5. Deploy

```bash
just infra-apply deploy-baba    # Provision Lambda, EFS, S3
just deploy deploy-baba         # Build + push + update Lambda
just ui-open deploy-baba        # Open the live site
```
