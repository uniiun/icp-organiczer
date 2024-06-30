<script setup>
import { ref } from 'vue';
import { organiczer_backend } from 'declarations/organiczer_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  await organiczer_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}
</script>
<!-- /* sprawdzam tu tylko czy git działa */ -->
<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <h1 class="text-blue-600 text-center">Tailwind działa!!!</h1>
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>
