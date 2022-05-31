import "./setup";
import { Then } from "@cucumber/cucumber";
import { createReceiverActor } from "../../src/scripts/declarations";
import { expect } from "chai";

Then(/^Check receiver's notification count should be "([^"]*)"$/, async function (count) {
  const receiver = createReceiverActor()
  const notificationCount = await receiver.notificationCount();
  expect(notificationCount).to.equal(BigInt(count));
});