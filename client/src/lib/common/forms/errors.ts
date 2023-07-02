const parseField = (field?: string) =>
  field ? field[0].toUpperCase() + field.slice(1) : "This field";

export const errors = {
  min(n: number, field?: string) {
    field = parseField(field);

    return `${field} must contain at least ${n} character${n === 1 ? "" : "s"}`;
  },
  max(n: number, field?: string) {
    field = parseField(field);

    return `${field} must contain at most ${n} character${n === 1 ? "" : "s"}`;
  },
  confirm() {
    return "The passwords should match";
  },
  invalidCredentials() {
    return "TODO";
  },
};
