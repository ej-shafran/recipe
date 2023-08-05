/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import * as fc from "fast-check";
import { fakeUser } from "../support/fake";

export const SELECTORS = {
  HEADER: "[data-cy=BROWSE_HEADER]",
  RECIPE_PREVIEW: "[data-cy=RECIPE_PREVIEW]",
  TITLE_INPUT: "[data-cy=TITLE]",
  TITLE_ERROR: "[data-cy=TITLE_ERROR]",
  CONTENT_INPUT: "[data-cy=CONTENT]",
  SUBMIT_BUTTON: "[data-cy=SUBMIT]",
  RECIPE_TITLE: "[data-cy=RECIPE_TITLE]",
  RECIPE_CONTENT: "[data-cy=RECIPE_CONTENT]",
  RECIPE_PREVIEW_LINK: "[data-cy=RECIPE_PREVIEW] a",
  DELETE_RECIPE: "[data-cy=DELETE_RECIPE]",
  LOADING: "[data-cy=LOADING]",
};

const RECIPE_LIMIT = 10;

type Recipe = {
  content: string;
  title: string;
};

type Model = {
  recipes: number;
  deleteButtons: number;
};

const addRecipeCommand = (
  recipe: Recipe
): fc.ICommand<Model, typeof cy, void> => ({
  check() {
    return true;
  },

  run(m, r) {
    // Affect Model
    m.recipes++;

    // Affect Real
    r.get(SELECTORS.TITLE_INPUT).type(recipe.title, {
      parseSpecialCharSequences: false,
    });
    r.get(SELECTORS.CONTENT_INPUT).type(recipe.content, {
      parseSpecialCharSequences: false,
    });
    r.get(SELECTORS.SUBMIT_BUTTON).click();

    // Assert
    r.get(SELECTORS.RECIPE_TITLE).contains(recipe.title);
    r.visit("/browse");
    r.get(SELECTORS.RECIPE_PREVIEW).should(
      "have.length",
      Math.min(RECIPE_LIMIT, m.recipes)
    );
  },

  toString(): string {
    return `AddRecipe ${JSON.stringify(recipe, null, " ")}`;
  },
});

const deleteRecipeCommand = (
  i: number
): fc.ICommand<Model, typeof cy, void> => ({
  check(m) {
    return i < m.deleteButtons;
  },
  run(m, r) {
    // Affect Model
    if (m.recipes < RECIPE_LIMIT) {
      m.recipes--;
      m.deleteButtons--;
    }

    // Affect Real
    r.get(SELECTORS.DELETE_RECIPE).then((buttons) => buttons[i].click());

    // Assert
    if (m.recipes >= RECIPE_LIMIT) {
      r.get(SELECTORS.RECIPE_PREVIEW).should("have.length.within", RECIPE_LIMIT - 1, RECIPE_LIMIT);

      m.recipes = r.$$(SELECTORS.RECIPE_PREVIEW).length;
      m.deleteButtons = r.$$(SELECTORS.DELETE_RECIPE).length;
    } else {
      r.get(SELECTORS.RECIPE_PREVIEW).should("have.length", m.recipes);
    }
  },
  toString() {
    return `DeleteRecipe (${i})`;
  },
});

describe("Browse Recipe Page", () => {
  let model: Model;
  before(() => {
    const user = fakeUser();
    cy.request("POST", "/api/user/register", user);
    cy.login(user.username, user.password);
    cy.visit("/browse");
    cy.get(SELECTORS.LOADING)
      .should("not.exist")
      .then(() => {
        model = {
          recipes: cy.$$(SELECTORS.RECIPE_PREVIEW).length,
          deleteButtons: cy.$$(SELECTORS.DELETE_RECIPE).length,
        };
      });
  });

  it("test domain logic", () => {
    const recipeArb = fc.record({
      content: fc.string({ minLength: 30 }),
      title: fc.string({ minLength: 30 }),
    });

    const addRecipeArb = recipeArb.map(addRecipeCommand);
    const deleteRecipeArb = fc
      .integer({ min: 0, max: model.deleteButtons })
      .map(deleteRecipeCommand);

    const commands = [addRecipeArb, deleteRecipeArb];
    const commandsArb = fc.commands(commands, { size: "+1" });

    const prop = fc.property(commandsArb, (cmds) => {
      fc.modelRun(
        () => ({
          model,
          real: cy,
        }),
        cmds
      );
    });

    fc.assert(prop, { numRuns: 1 });
  });
});
