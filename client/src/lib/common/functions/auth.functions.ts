import cookieJar from "js-cookie";
import decode from "jwt-decode";

const AUTH_COOKIE = import.meta.env.VITE_AUTH_COOKIE;

if (!AUTH_COOKIE) {
  throw new Error("No `AUTH_COOKIE` set in `import.meta.env`!");
}

export function getUserId() {
  const cookie = cookieJar.get(AUTH_COOKIE);

  return cookie ? decode<{ id: string }>(cookie).id : null;
}

export function isAuthenticated() {
  return !!getUserId();
}
