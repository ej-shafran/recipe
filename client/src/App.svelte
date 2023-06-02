<script lang="ts">
  import { Link, Route, Router } from "svelte-navigator";

  import Index from "./lib/routes/Index.svelte";
  import Login from "./lib/routes/Login.svelte";
  import Register from "./lib/routes/Register.svelte";
  import Recipe from "./lib/routes/Recipe.svelte";
  import Profile from "./lib/routes/Profile.svelte";
  import Home from "./lib/routes/Home.svelte";
  import Browse from "./lib/routes/Browse.svelte";

  const routes = [
    "/browse",
    "/register",
    "/login",
    "/recipe/:id",
    "/user/:id/profile",
    "/user/:id",
  ];
</script>

<Router>
  <nav>
    {#each routes as to}
      <Link {to}>{to}</Link>
    {/each}
  </nav>

  <Route path="/" component={Index} />

  <Route path="/login">
    <Login />
  </Route>
  <Route path="/register">
    <Register />
  </Route>
  <Route path="/recipe/:id" let:params>
    <Recipe id={params.id}/>
  </Route>
  <Route path="/user/:id/profile" let:params>
    <Profile id={params.id}/>
  </Route>
  <Route path="/user/:id" let:params>
    <Home id={params.id}/>
  </Route>
  <Route path="/browse">
    <Browse />
  </Route>

  <Route>
    <h1>Error 404: Page Not Found</h1>
  </Route>
</Router>

<style>
  nav {
    display: flex;
    gap: 1rem;
  }
</style>
