import * as fc from "fast-check";
import { fakeUser } from "../support/fake";

const SELECTORS = {
  RECIPE_PREVIEW_LINK: "[data-cy=RECIPE_PREVIEW] a",
  COMMENTS: "[data-cy=RECIPE_COMMENT]",
  NEW_COMMENT_INPUT: "[data-cy=CONTENT]",
  SUBMIT_BUTTON: "[data-cy=SUBMIT]",
};

const RECIPE_URL_REGEX = /\/recipe\/\d+/;
const COMMENT_LIMIT = 10;

const user = fakeUser();

describe("Recipe details page", () => {
  before(() => {
    cy.request("POST", "/api/user/register", user);
    cy.login(user.username, user.password);
  });

  it("shows comments and allows adding them", () => {
    const commentArb = fc.string({ minLength: 10 });

    const prop = fc.property(commentArb, (comment) => {
      cy.get(SELECTORS.RECIPE_PREVIEW_LINK).first().click();
      cy.url().should("match", RECIPE_URL_REGEX);

      cy.get(SELECTORS.COMMENTS).should("have.length.lte", COMMENT_LIMIT);
      cy.get(SELECTORS.NEW_COMMENT_INPUT).type(comment, {
        parseSpecialCharSequences: false,
      });
      cy.get(SELECTORS.SUBMIT_BUTTON).click();

      cy.get(SELECTORS.COMMENTS).should("contain", user.username);
      cy.get(SELECTORS.COMMENTS).should("contain", comment);
    });

    fc.assert(
      prop.beforeEach(() => cy.visit("/browse")),
      { numRuns: 5 }
    );
  });
});
