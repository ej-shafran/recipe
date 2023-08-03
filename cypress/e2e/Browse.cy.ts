/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import * as fc from "fast-check";
import { TEST_USER } from "../support/constants";

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
};

const VALUES = {
  TITLE:
    "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
  CONTENT:
    "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod.",
};

const RECIPE_LIMIT = 10;

type Recipe = {
  content: string;
  title: string;
};

type Model = {
  recipes: number;
};

export const addRecipeCommand = (
  recipe: Recipe
): fc.ICommand<Model, typeof cy, void> => ({
  check() {
    return true;
  },

  run(m: Model, r: typeof cy) {
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

    // TODO
    // r.get(SELECTORS.RECIPE_TITLE).contains(recipe.title);

    // Assert
    r.visit("/browse");
    r.get(SELECTORS.RECIPE_PREVIEW).should(
      "have.length",
      Math.min(RECIPE_LIMIT, m.recipes)
    );
  },

  toString(): string {
    return `${addRecipeCommand.name} ${JSON.stringify(recipe, null, " ")}`;
  },
});

describe("Browse Recipe Page", () => {
  let model: Model;

  before(() => {
    cy.login(TEST_USER.USERNAME, TEST_USER.PASSWORD);
    cy.visit("/browse");
    cy.get(SELECTORS.RECIPE_PREVIEW).then((recipes) => {
      model = { recipes: Math.min(RECIPE_LIMIT, recipes.length) };
    });
  });

  it.only("test domain logic", () => {
    const recipeArb = fc.record({
      content: fc.string({ minLength: 30 }),
      title: fc.string({ minLength: 30 }),
    });

    const addRecipeArb = recipeArb.map(addRecipeCommand);

    const commands = [addRecipeArb];
    const commandsArb = fc.commands(commands, { size: "small" });

    const prop = fc.property(commandsArb, (cmds) => {
      fc.modelRun(() => {
        return {
          model: model,
          real: cy,
        };
      }, cmds);
    });

    fc.assert(prop, { numRuns: 1 });
  });
});

export { };
