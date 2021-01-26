import * as Turbo from "@hotwired/turbo"
import { Application } from "stimulus";
import { definitionsFromContext } from "stimulus/webpack-helpers";

const application = Application.start();
const context = require.context("./controllers", true, /_controller\.ts$/);
application.load(definitionsFromContext(context));
const ws = new WebSocket("ws://localhost:8001")
Turbo.connectStreamSource(ws);