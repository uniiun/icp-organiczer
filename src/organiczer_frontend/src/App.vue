<script setup>
import { ref } from 'vue';
import { organiczer_backend } from 'declarations/organiczer_backend/index';
import Calendar from './Components/Calendar.vue';

// let greeting = ref('');

// async function handleSubmit(e) {
//   e.preventDefault();
//   const target = e.target;
//   const name = target.querySelector('#name').value;
//   await organiczer_backend.greet(name).then((response) => {
//     greeting.value = response;
//   });
// }
</script>

<template>
  <main>
    <h1 class="text-blue-600 text-center">Tailwind działa!!!</h1>
    <br />
    <Calendar />

    
  </main>
</template>
