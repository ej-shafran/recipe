/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import { TEST_USER } from "../support/constants";

const SELECTORS = {
  HEADER: "[data-cy=BROWSE_HEADER]",
  RECIPE_PREVIEW: "[data-cy=RECIPE_PREVIEW]",
  TITLE_INPUT: "[data-cy=TITLE]",
  TITLE_ERROR: "[data-cy=TITLE_ERROR]",
  CONTENT_INPUT: "[data-cy=CONTENT]",
  SUBMIT_BUTTON: "[data-cy=SUBMIT]",
  RECIPE_TITLE: "[data-cy=RECIPE_TITLE]",
  RECIPE_CONTENT: "[data-cy=RECIPE_CONTENT]",
  RECIPE_PREVIEW_LINK: "[data-cy=RECIPE_PREVIEW] a",
};

const VALUES = {
  TITLE:
    "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
  CONTENT:
    "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod.",
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
    cy.get(SELECTORS.RECIPE_CONTENT)
      .should("exist")
      .should("contain", VALUES.CONTENT);
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
