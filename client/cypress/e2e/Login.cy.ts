/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import { errors } from "../../src/lib/common/forms/errors";

const SELECTORS = (() => {
  const PARENT = "[data-cy=LOGIN_FORM]";

  return {
    USERNAME_INPUT: `${PARENT} [data-cy=USERNAME]`,
    USERNAME_ERROR: `${PARENT} [data-cy=USERNAME_ERROR]`,
    PASSWORD_INPUT: `${PARENT} [data-cy=PASSWORD]`,
    PASSWORD_ERROR: `${PARENT} [data-cy=PASSWORD_ERROR]`,
    SUBMIT_BUTTON: `${PARENT} [data-cy=SUBMIT]`,
    HOME_HEADER: `[data-cy=HOME_HEADER]`,
  };
})();

const INVALID_VALUES = {
  TOO_SHORT: "a1!",
  TOO_LONG: "a1!".repeat(34),
};
const TEST_VALUES = {
  USERNAME: "TEST_USER",
  PASSWORD: "12345678",
};
const NONEXISTENT_VALUES = {
  USERNAME: "INVALID",
  PASSWORD: "DOESNTEXIST",
};

const LOGIN_ROUTE = {
  method: "POST",
  url: "/api/user/login",
};

const MIN_LENGTH = 5;
function createSuite(field: "USERNAME" | "PASSWORD", max: number) {
  const INPUT = `${field}_INPUT`;
  const ERROR = `${field}_ERROR`;

  return () => {
    it(`should display an error when the ${field.toLowerCase()} is too short`, () => {
      cy.get(SELECTORS[INPUT]).type(INVALID_VALUES.TOO_SHORT).blur();
      cy.get(SELECTORS[ERROR]).should("contain", errors.min(MIN_LENGTH));

      cy.get(SELECTORS[INPUT]).clear().type(TEST_VALUES[field]).blur();
      cy.get(SELECTORS[ERROR]).should("be.empty");
    });

    it(`should display an error when the ${field.toLowerCase()} is too long`, () => {
      cy.get(SELECTORS[INPUT]).type(INVALID_VALUES.TOO_LONG).blur();
      cy.get(SELECTORS[ERROR]).should("contain", errors.max(max));

      cy.get(SELECTORS[INPUT]).clear().type(TEST_VALUES[field]).blur();
      cy.get(SELECTORS[ERROR]).should("be.empty");
    });
  };
}

describe("Login Page", () => {
  beforeEach(() => cy.visit("/login"));

  describe("invalid username", createSuite("USERNAME", 30));
  describe("invalid password", createSuite("PASSWORD", 100));

  describe("invalid credentials", () => {
    it("should display an error under the username when the credentials are wrong", () => {
      cy.get(SELECTORS.USERNAME_INPUT).type(NONEXISTENT_VALUES.USERNAME);
      cy.get(SELECTORS.PASSWORD_INPUT).type(NONEXISTENT_VALUES.PASSWORD);
      cy.get(SELECTORS.SUBMIT_BUTTON).click();

      cy.get(SELECTORS.USERNAME_ERROR).should(
        "contain",
        errors.invalidCredentials()
      );
    });
  });

  describe("successful login", () => {
    it("should correctly log in when correct credentials are used", () => {
      cy.login(TEST_VALUES.USERNAME, TEST_VALUES.PASSWORD);
      cy.get(SELECTORS.HOME_HEADER).should("exist");
    });
  });
});
