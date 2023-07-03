/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

const FORM = "[data-cy=LOGIN_FORM]";

const SELECTORS = {
  USERNAME_INPUT: `${FORM} [data-cy=USERNAME]`,
  USERNAME_ERROR: `${FORM} [data-cy=USERNAME_ERROR]`,
  PASSWORD_INPUT: `${FORM} [data-cy=PASSWORD]`,
  SUBMIT_BUTTON: `${FORM} [data-cy=SUBMIT]`,
  HOME_HEADER: `[data-cy=HOME_HEADER]`,
};

const TEST_VALUES = {
  USERNAME: "TEST_USER",
  PASSWORD: "12345678",
};
const NONEXISTENT_VALUES = {
  USERNAME: "INVALID",
  PASSWORD: "DOESNTEXIST",
};

const INVALID_CREDENTIALS = "Invalid credentials. Please try again.";

describe("Login Page", () => {
  beforeEach(() => cy.visit("/login"));

  it("should display an error under the username when the credentials are wrong", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(NONEXISTENT_VALUES.USERNAME);
    cy.get(SELECTORS.PASSWORD_INPUT).type(NONEXISTENT_VALUES.PASSWORD);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.USERNAME_ERROR).should("contain", INVALID_CREDENTIALS);
  });

  it("should correctly log in when correct credentials are used", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(TEST_VALUES.USERNAME);
    cy.get(SELECTORS.PASSWORD_INPUT).type(TEST_VALUES.PASSWORD);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.HOME_HEADER).should("exist");
  });
});
