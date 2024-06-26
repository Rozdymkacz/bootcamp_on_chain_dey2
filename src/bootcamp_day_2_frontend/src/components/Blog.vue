<template>
      <div>
                  <h2 class="text-green-600"> Wpisy na bloga:</h2>
                  <div class="w-100 flex flex-row-reverse">
                        <button @click="pobierzWpisy" class="bg-blue-600 rounded-md text-white p-4">odświerz</button>
                  </div>
                  <div  class="grid mx-6 gap-4 my-4">
                        <div v-for="(wpis, index) in wpisy"  class="drop-shadow-xl bg-stone-300 p-4">
                              <p>{{ wpis }}</p>
                              <p>id: {{ index }}</p>
                              <button @click="usunWpis(index)" class="bg-blue-600 rounded-md text-white p-4">usun wpis</button>
                        </div> 
                  </div>
                  <div class="flex justiy-center flex-col">
                        <input v-model="nowyBlog" type="text" class="border-2 border-blue-600 p-4">
                        <button @click="dodajWpis" class="bg-blue-600 rounded-md text-white p-4">dodaj wpis</button>
                  </div>
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
                  await this.pobierzWpisy()
            },
            async pobierzWpisy(){
                  this.wpisy = await bootcamp_day_2_backend.oddaj_wpisy();
            },
            async usunWpis(index){
                  await bootcamp_day_2_backend.usun_wpis(index);
                  await this.pobierzWpisy();
            }
      },
      async mounted() {
            this.pobierzWpisy()
      }
}
</script>