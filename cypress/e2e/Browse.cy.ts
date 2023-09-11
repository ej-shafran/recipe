/// <reference types="cypress" />
/// <reference path="../support/commands.ts" />

import * as fc from "fast-check";
import { faker } from "@faker-js/faker";

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

type Recipe = {
  content: string;
  title: string;
};

type Model = {
  recipes: number;
  added: number;
};

let model: Model;

function AddRecipe(recipe: Recipe): fc.ICommand<Model, never, void> {
  return {
    check() {
      return true;
    },
    toString() {
      return `AddRecipe (${JSON.stringify(recipe)})`;
    },
    run(m) {
      // Log
      cy.log(this.toString());

      // Affect model
      m.recipes++;
      m.added++;
      model = m;

      // Affect real
      cy.get(SELECTORS.TITLE_INPUT).type(recipe.title, {
        parseSpecialCharSequences: false,
        log: false,
      });
      cy.get(SELECTORS.CONTENT_INPUT).type(recipe.content, {
        parseSpecialCharSequences: false,
        log: false,
      });
      cy.get(SELECTORS.SUBMIT_BUTTON).click();
      cy.visit("/browse");
      cy.get(SELECTORS.LOADING).should("not.exist");
    },
  };
}

const DeleteRecipe = (): fc.ICommand<Model, never, void> => ({
  check(m) {
    return m.added > 0;
  },
  toString() {
    return "DeleteRecipe";
  },
  run(m) {
    // Log
    cy.log(this.toString());

    // Affect model
    m.recipes--;
    m.added--;
    model = m;

    // Affect real
    cy.get(SELECTORS.DELETE_RECIPE).then((deleteButtons) => {
      const button = faker.helpers.arrayElement(deleteButtons.toArray());
      button.click();
    });
  },
});

describe("Browse Recipe Page", () => {
  before(() => {
    const user = fakeUser();
    cy.request("POST", "/api/user/register", user);
    cy.login(user.username, user.password);

    cy.request("GET", "/api/recipe/count").then((response) => {
      model = {
        recipes: response.body,
        added: 0,
      };
    });

    cy.visit("/browse");
  });

  it("tests domain logic", () => {
    const commands = [
      fc
        .record<Recipe>({
          content: fc.string({ minLength: 30 }),
          title: fc.string({ minLength: 30 }),
        })
        .map(AddRecipe),
      fc.constant(null).map(DeleteRecipe),
    ];

    const prop = fc.property(fc.commands(commands), (cmds) => {
      fc.modelRun(() => ({ model, real: undefined }), cmds);

      cy.wait(300);
      cy.request("GET", "/api/recipe/count").then((response) => {
        expect(response.body).to.equal(model.recipes);
      });
    });

    fc.assert(prop, { numRuns: 1 });
  });
});
