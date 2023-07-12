const TEST_USER = {
  USERNAME: "TEST_USER",
  PASSWORD: "12345678",
};

const SELECTORS = {
  RECIPE_PREVIEW_LINK: "[data-cy=RECIPE_PREVIEW] a",
  COMMENTS: "[data-cy=RECIPE_COMMENT]",
  NEW_COMMENT_INPUT: "[data-cy=CONTENT]",
  SUBMIT_BUTTON: "[data-cy=SUBMIT]",
};

const RECIPE_URL_REGEX = /\/recipe\/\d+/;
const COMMENT_LIMIT = 10;

function randomComment() {
  return crypto.randomUUID();
}

describe("Recipe details page", () => {
  it("shows comments and allows adding them", () => {
    cy.login(TEST_USER.USERNAME, TEST_USER.PASSWORD);
    cy.visit("/browse");

    cy.get(SELECTORS.RECIPE_PREVIEW_LINK).first().click();
    cy.url().should("match", RECIPE_URL_REGEX);

    const comment = randomComment();
    cy.get(SELECTORS.COMMENTS).should("have.length.lte", COMMENT_LIMIT);
    cy.get(SELECTORS.NEW_COMMENT_INPUT).type(comment);
    cy.get(SELECTORS.SUBMIT_BUTTON).click();

    cy.get(SELECTORS.COMMENTS).should("contain", TEST_USER.USERNAME);
    cy.get(SELECTORS.COMMENTS).should("contain", comment);
  });
});
