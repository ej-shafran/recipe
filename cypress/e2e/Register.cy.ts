/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

const FORM = "[data-cy=REGISTER_FORM]";

const SELECTORS = {
  USERNAME_INPUT: `${FORM} [data-cy=USERNAME]`,
  USERNAME_ERROR: `${FORM} [data-cy=USERNAME_ERROR]`,
  PASSWORD_INPUT: `${FORM} [data-cy=PASSWORD]`,
  CONFIRM_PASSWORD_INPUT: `${FORM} [data-cy=CONFIRMPASSWORD]`,
  SUBMIT_BUTTON: `${FORM} [data-cy=SUBMIT]`,
  HOME_HEADER: `[data-cy=HOME_HEADER]`,
};

const EXISTING_VALUES = {
  USERNAME: "TEST_USER",
  PASSWORD: "12345678",
};

const NEW_VALUES = {
  USERNAME: "NEW_USER",
  PASSWORD: "abcdefgh",
};

const INVALID_CREDENTIALS = "Invalid credentials. Please try again.";

describe("Register Page", () => {
  before(() => {
    if (!cy.isCI()) {
      cy.exec("sqlx database reset -y --source ./server/migrations/test");
    }
  });

  beforeEach(() => cy.visit("/register"));

  it("should display an error when an existing user is entered", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(EXISTING_VALUES.USERNAME);
    cy.get(SELECTORS.PASSWORD_INPUT).type(EXISTING_VALUES.PASSWORD);
    cy.get(SELECTORS.CONFIRM_PASSWORD_INPUT).type(EXISTING_VALUES.PASSWORD);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.USERNAME_ERROR).should("contain", INVALID_CREDENTIALS);
  });

  it("should successfully add a new user with new values", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(NEW_VALUES.USERNAME);
    cy.get(SELECTORS.PASSWORD_INPUT).type(NEW_VALUES.PASSWORD);
    cy.get(SELECTORS.CONFIRM_PASSWORD_INPUT).type(NEW_VALUES.PASSWORD);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.USERNAME_ERROR).should("not.contain", INVALID_CREDENTIALS);
    cy.login(NEW_VALUES.USERNAME, NEW_VALUES.PASSWORD)
      .its("status")
      .should("equal", 200);
    cy.get(SELECTORS.HOME_HEADER).should("exist");
  });
});

export { };
