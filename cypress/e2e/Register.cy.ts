/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import { TEST_USER } from "../support/constants";

const FORM = "[data-cy=REGISTER_FORM]";

const SELECTORS = {
  USERNAME_INPUT: `${FORM} [data-cy=USERNAME]`,
  USERNAME_ERROR: `${FORM} [data-cy=USERNAME_ERROR]`,
  PASSWORD_INPUT: `${FORM} [data-cy=PASSWORD]`,
  CONFIRM_PASSWORD_INPUT: `${FORM} [data-cy=CONFIRMPASSWORD]`,
  SUBMIT_BUTTON: `${FORM} [data-cy=SUBMIT]`,
  HOME_HEADER: `[data-cy=HOME_HEADER]`,
};

const NEW_USER = {
  USERNAME: "NEW_USER",
  PASSWORD: "abcdefgh",
};

const INVALID_CREDENTIALS = "Invalid credentials. Please try again.";

describe("Register Page", () => {
  before(() => cy.resetDB());

  beforeEach(() => cy.visit("/register"));

  it("should display an error when an existing user is entered", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(TEST_USER.USERNAME);
    cy.get(SELECTORS.PASSWORD_INPUT).type(TEST_USER.PASSWORD);
    cy.get(SELECTORS.CONFIRM_PASSWORD_INPUT).type(TEST_USER.PASSWORD);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.USERNAME_ERROR).should("contain", INVALID_CREDENTIALS);
  });

  it("should successfully add a new user with new values", () => {
    cy.intercept("POST", "/api/user/register").as("register");

    cy.get(SELECTORS.USERNAME_INPUT).type(NEW_USER.USERNAME);
    cy.get(SELECTORS.PASSWORD_INPUT).type(NEW_USER.PASSWORD);
    cy.get(SELECTORS.CONFIRM_PASSWORD_INPUT).type(NEW_USER.PASSWORD);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.wait("@register");

    cy.login(NEW_USER.USERNAME, NEW_USER.PASSWORD)
      .its("status")
      .should("equal", 200);
    cy.get(SELECTORS.HOME_HEADER).should("exist");
  });
});

export { };
