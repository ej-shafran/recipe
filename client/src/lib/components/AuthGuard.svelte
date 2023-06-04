<script lang="ts">
  import { isAuthenticated } from "../common/functions/auth.functions";
  import { useLocation, navigate } from "svelte-navigator";
  import { onDestroy } from "svelte";

  export let privateRoutes: string[];
  export let redirect = "/";

  const routeRegexes = privateRoutes.map((route) => {
    const splitRoute = route.split("/");
    const routeFormat = splitRoute
      .map((section) => {
        if (section.startsWith(":")) {
          return "[^/]*";
        } else {
          return section;
        }
      })
      .join("/");

    return new RegExp(routeFormat);
  });

  const location = useLocation();
  const unsub = location.subscribe(({ pathname }) => {
    const hasPermission =
      !isAuthenticated() && routeRegexes.some((regex) => regex.test(pathname));

    if (hasPermission) {
      navigate(redirect, { replace: true });
    }
  });

  onDestroy(unsub);
</script>
