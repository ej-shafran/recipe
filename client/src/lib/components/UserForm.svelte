<script lang="ts">
  import type {
    UserValues,
    UserFormErrors,
  } from "../common/schemas/user.schema";
  import { userSchema } from "../common/schemas/user.schema";

  export let submit: (values: UserValues) => void;
  export let buttonText: string;

  const touched = {
    username: false,
    password: false,
  };

  let errors: UserFormErrors = {};

  let username = "";
  let password = "";

  function handleSubmit() {
    if (!validate(username, password)) return;

    submit({ username, password });
  }

  function validate(username: unknown, password: unknown) {
    const validation = userSchema.safeParse({ username, password });

    if (!validation.success) {
      errors = validation.error.format();
    } else {
      errors = {};
    }

    return validation.success;
  }

  function touch(key: keyof UserValues) {
    if (!touched[key]) touched[key] = true;
  }

  $: validate(username, password);
</script>

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

  <button type="submit">{buttonText}</button>
</form>
