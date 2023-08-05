/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import * as fc from "fast-check";

type User = {
  username: string;
  password: string;
};

const FORM = "[data-cy=REGISTER_FORM]";

const SELECTORS = {
  USERNAME_INPUT: `${FORM} [data-cy=USERNAME]`,
  USERNAME_ERROR: `${FORM} [data-cy=USERNAME_ERROR]`,
  PASSWORD_INPUT: `${FORM} [data-cy=PASSWORD]`,
  CONFIRM_PASSWORD_INPUT: `${FORM} [data-cy=CONFIRMPASSWORD]`,
  SUBMIT_BUTTON: `${FORM} [data-cy=SUBMIT]`,
  HOME_HEADER: `[data-cy=HOME_HEADER]`,
};

function fillInForm(user: { username: string; password: string }) {
  cy.get(SELECTORS.USERNAME_INPUT).type(user.username);
  cy.get(SELECTORS.PASSWORD_INPUT).type(user.password);
  cy.get(SELECTORS.CONFIRM_PASSWORD_INPUT).type(user.password);
  cy.get(SELECTORS.SUBMIT_BUTTON).click();
}

const INVALID_CREDENTIALS = "Invalid credentials. Please try again.";

describe("Register Page", () => {
  it("should properly create users in the database", () => {
    const userArb = fc.record<User>({
      username: fc.base64String({ minLength: 20, maxLength: 30 }),
      password: fc.base64String({ minLength: 20 }),
    });

    const prop = fc.property(userArb, (user) => {
      fillInForm(user);
      // we can't reset a request interception so we're forced to use milliseconds
      cy.wait(1000);
      cy.login(user.username, user.password).its("status").should("equal", 200);
      cy.get(SELECTORS.HOME_HEADER).should("exist");

      cy.visit("/register");
      fillInForm(user);
      cy.get(SELECTORS.USERNAME_ERROR).should("contain", INVALID_CREDENTIALS);
    });

    fc.assert(
      prop.beforeEach(() => cy.visit("/register")),
      { numRuns: 5 }
    );
  });
});
