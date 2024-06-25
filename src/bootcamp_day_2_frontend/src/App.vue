<script setup>
import { ref } from 'vue';
import { bootcamp_day_2_backend } from 'declarations/bootcamp_day_2_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const last_name = target.querySelector('#last_name').value;
  const numer = target.querySelector('#number').value;
  await bootcamp_day_2_backend.greet(name, last_name, Number(numer)).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <input id="last_name" alt="Name" type="text" />
      <input id="number" alt="Number" type="number" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>
