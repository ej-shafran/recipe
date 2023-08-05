import { faker } from "@faker-js/faker";
import * as fc from "fast-check";

export type User = {
  username: string;
  password: string;
};

export const unbiasedInt = fc.integer().noBias().noShrink();

export const fakeUser = (): User => ({
  username: faker.internet.userName(),
  password: faker.internet.password(),
});

export const fakerToArb = <T>(fakerGen: () => T) => {
  return unbiasedInt.map((seed) => {
    faker.seed(seed);
    return fakerGen();
  });
};
