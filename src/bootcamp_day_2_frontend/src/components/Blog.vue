<template>
      <div>
            <h1>
                  <button @click="pobierzWpisy">oddaj wpisy</button>
                  siema
                  {{ wpisy }}
                  <input v-model="nowyBlog" type="text">
                  <button @click="dodajWpis">dodaj wpis</button>
            </h1>
      </div>
</template>

<script>
import { bootcamp_day_2_backend } from 'declarations/bootcamp_day_2_backend/index';

export default {
      data() {
            return{
                 wpisy: [],
                 nowyBlog: "" 
            }
      },
      methods: {
            async dodajWpis(){
                  await bootcamp_day_2_backend.dodaj_wpis(this.nowyBlog);
            },
            async pobierzWpisy(){
                  this.wpisy = await bootcamp_day_2_backend.oddaj_wpisy();
            }
      }
}
</script>