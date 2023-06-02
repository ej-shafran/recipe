<script lang="ts">
  import { z } from "zod";
  import axios, { AxiosError } from "axios";
  import { navigate } from "svelte-navigator";

  const schema = z.object({
    username: z.string().min(5),
    password: z.string().min(5),
  });

  type Form = z.infer<typeof schema>;

  let errors = {} as z.ZodFormattedError<Form>;
  let username = "";
  let password = "";

  const touched = {
    username: false,
    password: false,
  };

  async function handleSubmit() {
    if (!validate(username, password)) return;

    try {
      await axios.post("/api/login", { username, password });
      // TODO: handle successful login
    } catch (error) {
      if (error instanceof AxiosError) {
        if (error.status === 401) {
          // TODO: call some sort of pop-up here!
        } else {
          // TODO: handle this route, handle some known status codes
          return navigate(`/error`);
        }
      }

      navigate(`/error/Something Went Wrong`);
    }
  }

  function validate(username: unknown, password: unknown) {
    const validation = schema.safeParse({ username, password });

    if (!validation.success) {
      errors = validation.error.format();
    } else {
      errors = {} as z.ZodFormattedError<Form>;
    }

    return validation.success;
  }

  function touch(key: keyof typeof touched) {
    if (!touched[key]) touched[key] = true;
  }

  $: validate(username, password);
</script>

<h1>Login Page</h1>

<form on:submit|preventDefault={handleSubmit}>
  <div>
    <label for="username"> Username: </label>
    <input
      id="username"
      type="text"
      bind:value={username}
      on:blur={() => touch("username")}
    />
    <span>
      {#if touched.username}
        {errors?.username?._errors ?? ""}
      {/if}
    </span>
  </div>

  <div>
    <label for="password"> Password: </label>
    <input
      id="password"
      type="password"
      bind:value={password}
      on:blur={() => touch("password")}
    />
    <span>
      {#if touched.password}
        {errors?.password?._errors ?? ""}
      {/if}
    </span>
  </div>

  <button type="submit">Log In</button>
</form>
