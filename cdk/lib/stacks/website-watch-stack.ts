import path = require("path");
import dotenv = require("dotenv");

import { Duration, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";
import { Rule, Schedule } from "aws-cdk-lib/aws-events";
import { LambdaFunction } from "aws-cdk-lib/aws-events-targets";
import { Code, Function, Runtime } from "aws-cdk-lib/aws-lambda";
import { Topic } from "aws-cdk-lib/aws-sns";

const LAMBDA_ASSET_LOCATION = "../../../lambda/target/lambda/website_watch/bootstrap.zip";

export interface WebsiteWatchStackProps extends StackProps {}

export class WebsiteWatchStack extends Stack {
    private readonly snsTopic: Topic;
    private readonly notifierLambda: Function;
    private readonly scheduleRule: Rule;

    constructor(scope: Construct, id: string, props: WebsiteWatchStackProps) {
        super(scope, id);

        this.snsTopic = this.createSnsTopic();
        this.notifierLambda = this.createNotifierLambda(this.snsTopic.topicArn);
        this.scheduleRule = this.createScheduleRule();

        this.createEventSourceRule(this.notifierLambda, this.scheduleRule);
        this.configurePermissions(this.notifierLambda, this.snsTopic);
    }

    private createSnsTopic(): Topic {
        return new Topic(this, "SmsTopic", {
            displayName: "Website Watch Topic",
            topicName: "WebsiteWatchTopic",
        });
    }

    private createNotifierLambda(snsTopicArn: string): Function {
        return new Function(this, "WebsiteWatchLambda", {
            code: Code.fromAsset(path.join(__dirname, LAMBDA_ASSET_LOCATION)),
            handler: "main.rs",
            runtime: Runtime.PROVIDED_AL2,
            environment: {
                "SNS_TOPIC_ARN": snsTopicArn
            }
        });
    }

    private createScheduleRule(): Rule {
        return new Rule(this, "WebsiteWatchSchedule", {
            description: "Check website for diff once every 12 hours",
            enabled: false,
            ruleName: "OnceEvery12h",
            schedule: Schedule.cron({
                hour: "*/12",
                minute: "0"
            })
        });
    }

    private createEventSourceRule(lambda: Function, rule: Rule) {
        rule.addTarget(new LambdaFunction(lambda, {}));
    }
    private configurePermissions(lambda: Function, topic: Topic) {
        topic.grantPublish(lambda);
    }
}