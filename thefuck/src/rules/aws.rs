use crate::types::{Command, MatchResult};
use crate::util;

const AWS_SERVICES: &[&str] = &[
    "s3",
    "ec2",
    "ecs",
    "eks",
    "lambda",
    "iam",
    "rds",
    "dynamodb",
    "cloudformation",
    "cloudwatch",
    "cloudfront",
    "elb",
    "vpc",
    "route53",
    "sns",
    "sqs",
    "kinesis",
    "emr",
    "glue",
    "athena",
    "redshift",
    "organizations",
    "config",
    "kms",
    "secretsmanager",
    "ssm",
    "codebuild",
    "codedeploy",
    "codepipeline",
    "ecr",
    "fargate",
    "apigateway",
    "stepfunctions",
    "sagemaker",
    "elasticache",
    "es",
    "acm",
    "waf",
    "shield",
    "guardduty",
    "cognito",
    "amplify",
    "appsync",
    "backup",
    "budgets",
    "ce",
    "directconnect",
    "dms",
    "eventbridge",
    "inspector",
    "lightsail",
    "mq",
    "neptune",
    "opensearch",
    "qldb",
    "rekognition",
    "ses",
    "snowball",
    "sts",
    "support",
    "swf",
    "transfer",
    "workspaces",
    "xray",
];

const AWS_SUBCOMMAND_TYPOS: &[(&str, &[&str])] = &[
    ("s3", &["s3s", "ss3", "s33"]),
    ("ec2", &["ecc2", "ec22"]),
    ("ecs", &["ecss", "ec"]),
    ("eks", &["ekss", "ek"]),
    ("iam", &["ima", "im"]),
    ("rds", &["rss", "rd"]),
    ("lambda", &["lambd", "lamba", "lambada"]),
];

const THRESHOLD: f64 = 0.70;

fn find_match(arg: &str) -> Option<(String, f64)> {
    util::fuzzy_match_arg(arg, AWS_SERVICES, AWS_SUBCOMMAND_TYPOS, THRESHOLD)
}

pub fn aws_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    let replacement = match command.parts[0].as_str() {
        "asw" | "a ws" | "awss" => "aws",
        _ => return None,
    };

    let mut corrected = command.parts.clone();
    corrected[0] = replacement.to_string();

    Some(MatchResult {
        rule: "aws_command",
        corrected_command: corrected.join(" "),
        similarity: util::SIMILARITY_TYPO,
    })
}

pub fn aws_service_typo_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.len() < 2 || command.parts[0] != "aws" {
        return None;
    }

    let arg = &command.parts[1];
    if arg.starts_with('-') || AWS_SERVICES.contains(&arg.as_str()) {
        return None;
    }

    let (corrected_sub, similarity) = find_match(arg)?;
    let mut corrected = command.parts.clone();
    corrected[1] = corrected_sub;

    Some(MatchResult {
        rule: "aws_service_typo",
        corrected_command: corrected.join(" "),
        similarity,
    })
}
