/// <reference types="cypress" />

const LOGIN_FORM_SELECTORS = (() => {
  const PARENT = "[data-cy=LOGIN_FORM]";

  return {
    USERNAME_INPUT: `${PARENT} [data-cy=USERNAME]`,
    PASSWORD_INPUT: `${PARENT} [data-cy=PASSWORD]`,
    SUBMIT_BUTTON: `${PARENT} [data-cy=SUBMIT]`,
  };
})();

// const REGISTER_FORM_SELECTORS = (() => {
//   const PARENT = "[data-cy=REGISTER_FORM]";
//
//   return {
//     USERNAME_INPUT: `${PARENT} [data-cy=USERNAME]`,
//     PASSWORD_INPUT: `${PARENT} [data-cy=PASSWORD]`,
//     CONFIRM_PASSWORD_INPUT: `${PARENT} [data-cy=CONFIRMPASSWORD]`,
//     SUBMIT_BUTTON: `${PARENT} [data-cy=SUBMIT]`,
//   };
// })();

Cypress.Commands.add("login", (username, password) => {
  return cy.request("POST", "/api/user/login", {
    username,
    password,
  });
});

Cypress.Commands.add("register", (username, password) => {
  return cy.request("POST", "/api/user/register", {
    username,
    password,
  });
});

Cypress.Commands.add("isCI", () => Cypress.env("CYPRESS_CI") !== "no");

declare global {
  namespace Cypress {
    interface Chainable {
      login(
        username: string,
        password: string
      ): Chainable<Cypress.Response<unknown>>;
      register(
        username: string,
        password: string
      ): Chainable<Cypress.Response<unknown>>;
      isCI(): boolean;
    }
  }
}

export { };
