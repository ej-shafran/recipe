/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import { faker } from "@faker-js/faker";
import { fakeUser, unbiasedInt } from "../support/fake";
import * as fc from "fast-check";

const FORM = "[data-cy=LOGIN_FORM]";

const SELECTORS = {
  USERNAME_INPUT: `${FORM} [data-cy=USERNAME]`,
  USERNAME_ERROR: `${FORM} [data-cy=USERNAME_ERROR]`,
  PASSWORD_INPUT: `${FORM} [data-cy=PASSWORD]`,
  SUBMIT_BUTTON: `${FORM} [data-cy=SUBMIT]`,
  HOME_HEADER: `[data-cy=HOME_HEADER]`,
};

const INVALID_CREDENTIALS = "Invalid credentials. Please try again.";

describe("Login Page", () => {
  it("should properly check a user's credentials", () => {
    const usersArb = fc.tuple(unbiasedInt, unbiasedInt).map(([a, b]) => {
      faker.seed(a);
      const user = fakeUser();
      faker.seed(b);
      const invalid = fakeUser();

      return {
        user,
        invalid,
      };
    });

    const prop = fc.property(usersArb, ({ user, invalid }) => {
      cy.request("POST", "/api/user/register", user);

      cy.get(SELECTORS.USERNAME_INPUT).type(invalid.username);
      cy.get(SELECTORS.PASSWORD_INPUT).type(invalid.password);
      cy.get(SELECTORS.SUBMIT_BUTTON).click();

      cy.get(SELECTORS.USERNAME_ERROR).should("contain", INVALID_CREDENTIALS);

      cy.reload();
      cy.get(SELECTORS.USERNAME_INPUT).type(user.username);
      cy.get(SELECTORS.PASSWORD_INPUT).type(user.password);
      cy.get(SELECTORS.SUBMIT_BUTTON).click();

      cy.get(SELECTORS.HOME_HEADER).should("exist");
    });

    fc.assert(
      prop.beforeEach(() => cy.visit("/login")),
      { numRuns: 5 }
    );
  });
});
