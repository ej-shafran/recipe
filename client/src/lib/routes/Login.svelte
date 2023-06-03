<script lang="ts">
  import axios from "axios";
  import UserForm from "../components/UserForm.svelte";
  import type { UserValues } from "../schemas/user.schema";

  async function handleSubmit(values: UserValues) {
    await axios.post("/api/login", values);
  }
</script>

<UserForm submit={handleSubmit} buttonText="Log In" />
