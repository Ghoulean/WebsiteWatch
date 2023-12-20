import { App } from "aws-cdk-lib";
import { WebsiteWatchStack } from "./stacks/website-watch-stack";

require("dotenv").config();

const app = new App();

const AWS_ENV_CONFIG = {};

new WebsiteWatchStack(app, "WebsiteWatchStack", {
  env: AWS_ENV_CONFIG,
});

app.synth();
