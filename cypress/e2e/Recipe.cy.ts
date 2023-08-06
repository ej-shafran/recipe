import * as fc from "fast-check";
import { fakeUser } from "../support/fake";

const SELECTORS = {
  RECIPE_PREVIEW_LINK: "[data-cy=RECIPE_PREVIEW] a",
  COMMENTS: "[data-cy=RECIPE_COMMENT]",
  NEW_COMMENT_INPUT: "[data-cy=CONTENT]",
  SUBMIT_BUTTON: "[data-cy=SUBMIT]",
  DELETE_COMMENT: "[data-cy=DELETE_COMMENT]",
  LOADING: "[data-cy=LOADING]",
};

const COMMENT_LIMIT = 10;

const user = fakeUser();

type Model = {
  comments: number;
  deleteButtons: number;
};

const addCommentCommand = (
  comment: string
): fc.ICommand<Model, typeof cy, void> => ({
  check() {
    return true;
  },

  run(m, r) {
    // Affect Model
    m.comments++;

    // Affect Real
    r.get(SELECTORS.NEW_COMMENT_INPUT).type(comment, {
      parseSpecialCharSequences: false,
    });
    r.get(SELECTORS.SUBMIT_BUTTON).click();

    // Assert
    r.get(SELECTORS.COMMENTS).contains(comment);
    r.scrollTo("bottom");
    r.get(SELECTORS.COMMENTS).should("have.length", m.comments);
  },

  toString(): string {
    return `AddComment ${JSON.stringify(comment, null, " ")}`;
  },
});

const deleteCommentCommand = (
  i: number
): fc.ICommand<Model, typeof cy, void> => ({
  check(m) {
    return i < m.deleteButtons;
  },

  run(m, r) {
    // Affect Model
    m.comments--;
    m.deleteButtons--;

    // Affect Real
    r.get(SELECTORS.DELETE_COMMENT).then((buttons) => buttons[i].click());

    // Assert
    r.scrollTo("bottom");
    r.get(SELECTORS.COMMENTS).should("have.length", m.comments);
  },

  toString() {
    return `DeleteRecipe (${i})`;
  },
});

describe("Recipe details page", () => {
  let model: Model;

  before(() => {
    cy.request("POST", "/api/user/register", user);
    cy.login(user.username, user.password);
    cy.visit("/browse");
    cy.get(SELECTORS.RECIPE_PREVIEW_LINK).first().click();
    // TODO: find a way to accurately get the count of commments,
    // TODO: or go back to limiting the model...
    cy.get(SELECTORS.LOADING)
      .should("not.exist")
      .then(() => {
        model = {
          comments: cy.$$(SELECTORS.COMMENTS).length,
          deleteButtons: cy.$$(SELECTORS.DELETE_COMMENT).length,
        };
      });
  });

  it("shows comments and allows adding them", () => {
    const commentArb = fc.string({ minLength: 10 });
    const addCommentArb = commentArb.map(addCommentCommand);
    const deleteCommentArb = fc.integer({ min: 0 }).map(deleteCommentCommand);

    const commands = [addCommentArb, deleteCommentArb];
    const commandsArb = fc.commands(commands);
    const prop = fc.property(commandsArb, (cmds) => {
      fc.modelRun(
        () => ({
          model,
          real: cy,
        }),
        cmds
      );
    });

    // TODO: add more runs and lower commands size
    fc.assert(prop, { numRuns: 1 });
  });
});
