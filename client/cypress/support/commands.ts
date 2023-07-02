/// <reference types="cypress" />

const LOGIN_FORM_SELECTORS = (() => {
  const PARENT = "[data-cy=LOGIN_FORM]";

  return {
    USERNAME_INPUT: `${PARENT} [data-cy=USERNAME]`,
    PASSWORD_INPUT: `${PARENT} [data-cy=PASSWORD]`,
    SUBMIT_BUTTON: `${PARENT} [data-cy=SUBMIT]`,
  };
})();

const REGISTER_FORM_SELECTORS = (() => {
  const PARENT = "[data-cy=REGISTER_FORM]";

  return {
    USERNAME_INPUT: `${PARENT} [data-cy=USERNAME]`,
    PASSWORD_INPUT: `${PARENT} [data-cy=PASSWORD]`,
    CONFIRM_PASSWORD_INPUT: `${PARENT} [data-cy=CONFIRMPASSWORD]`,
    SUBMIT_BUTTON: `${PARENT} [data-cy=SUBMIT]`,
  };
})();

Cypress.Commands.add("login", (username, password) => {
  cy.visit("/login");
  cy.get(LOGIN_FORM_SELECTORS.USERNAME_INPUT).type(username);
  cy.get(LOGIN_FORM_SELECTORS.PASSWORD_INPUT).type(password);
  cy.get(LOGIN_FORM_SELECTORS.SUBMIT_BUTTON).click();
});

Cypress.Commands.add("register", (username, password) => {
  cy.visit("/register");
  cy.get(REGISTER_FORM_SELECTORS.USERNAME_INPUT).type(username);
  cy.get(REGISTER_FORM_SELECTORS.PASSWORD_INPUT).type(password);
  cy.get(REGISTER_FORM_SELECTORS.CONFIRM_PASSWORD_INPUT).type(password);
  cy.get(REGISTER_FORM_SELECTORS.SUBMIT_BUTTON).click();
});

declare global {
  namespace Cypress {
    interface Chainable {
      login(username: string, password: string): Chainable<void>;
      register(username: string, password: string): Chainable<void>;
    }
  }
}

export { };
