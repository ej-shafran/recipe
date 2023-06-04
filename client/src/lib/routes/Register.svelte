<script lang="ts">
  import axios from "axios";
  import UserForm from "../components/UserForm.svelte";
  import type { UserValues } from "../schemas/user.schema";

  async function handleSubmit(values: UserValues) {
    await axios.post("/api/register", values);
  }
</script>

<h1>Register Page</h1>
<UserForm submit={handleSubmit} buttonText="Register" />
