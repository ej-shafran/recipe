/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import { faker } from "@faker-js/faker";

const FORM = "[data-cy=LOGIN_FORM]";

const SELECTORS = {
  USERNAME_INPUT: `${FORM} [data-cy=USERNAME]`,
  USERNAME_ERROR: `${FORM} [data-cy=USERNAME_ERROR]`,
  PASSWORD_INPUT: `${FORM} [data-cy=PASSWORD]`,
  SUBMIT_BUTTON: `${FORM} [data-cy=SUBMIT]`,
  HOME_HEADER: `[data-cy=HOME_HEADER]`,
};

const seed = faker.number.int();
faker.seed(seed);

const user = {
  username: faker.internet.userName(),
  password: faker.internet.password(),
};

faker.seed(seed + 1);
const invalid = {
  username: faker.internet.userName(),
  password: faker.internet.password(),
};

const INVALID_CREDENTIALS = "Invalid credentials. Please try again.";

describe("Login Page", () => {
  before(() => {
    // create the user
    cy.request("POST", "/api/user/register", user);
  });

  beforeEach(() => cy.visit("/login"));

  it("should display an error under the username when the credentials are wrong", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(invalid.username);
    cy.get(SELECTORS.PASSWORD_INPUT).type(invalid.password);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.USERNAME_ERROR).should("contain", INVALID_CREDENTIALS);
  });

  it("should correctly log in when correct credentials are used", () => {
    cy.get(SELECTORS.USERNAME_INPUT).type(user.username);
    cy.get(SELECTORS.PASSWORD_INPUT).type(user.password);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.HOME_HEADER).should("exist");
  });
});
