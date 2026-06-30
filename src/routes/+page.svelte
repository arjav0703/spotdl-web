<script lang="ts">
  let status = fetch('/api/health')
    .then((res) => res.json())
    .then((data) => data.status);
</script>

<h1> Hi </h1>
<p>Server status: {#await status then s}{s}{/await}</p>
