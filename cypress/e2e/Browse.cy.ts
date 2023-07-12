/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

const TEST_USER = {
  USERNAME: "TEST_USER",
  PASSWORD: "12345678",
};

const SELECTORS = {
  HEADER: "[data-cy=BROWSE_HEADER]",
  RECIPE_PREVIEW: "[data-cy=RECIPE_PREVIEW]",
  TITLE_INPUT: "[data-cy=TITLE]",
  TITLE_ERROR: "[data-cy=TITLE_ERROR]",
  CONTENT_INPUT: "[data-cy=CONTENT]",
  SUBMIT_BUTTON: "[data-cy=SUBMIT]",
  RECIPE_TITLE: "[data-cy=RECIPE_TITLE]",
  RECIPE_PREVIEW_LINK: "[data-cy=RECIPE_PREVIEW] a",
};

const VALUES = {
  TITLE: "a!1".repeat(5),
  CONTENT: "a!1 ".repeat(10),
};

const RECIPE_LIMIT = 10;

describe("Browse Recipe Page", () => {
  beforeEach(() => {
    cy.login(TEST_USER.USERNAME, TEST_USER.PASSWORD);
    cy.visit("/browse");
  });

  it("displays a page of recipe previews", () => {
    cy.get(SELECTORS.HEADER).should("exist");
    cy.get(SELECTORS.RECIPE_PREVIEW).should("have.length.lte", RECIPE_LIMIT);
  });

  it("adds a new recipe", () => {
    cy.get(SELECTORS.TITLE_INPUT).type(VALUES.TITLE);
    cy.get(SELECTORS.CONTENT_INPUT).type(VALUES.CONTENT);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();
    cy.get(SELECTORS.RECIPE_TITLE)
      .should("exist")
      .should("contain", VALUES.TITLE);
  });

  it("links to recipe details", () => {
    cy.get(SELECTORS.RECIPE_PREVIEW_LINK)
      .last()
      .click()
      .then((a) => {
        cy.get(SELECTORS.RECIPE_TITLE).should("contain", a.text());
      });
  });
});

export {};
