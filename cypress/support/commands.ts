/// <reference types="cypress" />

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
    }
  }
}

export { };
